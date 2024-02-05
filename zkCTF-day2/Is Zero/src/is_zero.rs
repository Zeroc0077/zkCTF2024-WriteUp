use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Assigned, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

trait NumericInstructions<F: Field>: Chip<F> {
    /// Variable representing a number.
    type Num;

    fn load_private(
        &self,
        layouter: impl Layouter<F>,
        v: &[Value<F>],
    ) -> Result<Vec<Self::Num>, Error>;

    fn is_zero(
        &self,
        layouter: impl Layouter<F>,
        a: &[Self::Num],
        b: &[Self::Num],
    ) -> Result<Vec<Self::Num>, Error>;
}

/// The chip that will implement our instructions! Chips store their own
/// config, as well as type markers if necessary.
pub struct FieldChip<F: Field> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}

/// Chip state is stored in a config struct. This is generated by the chip
/// during configuration, and then stored inside the chip.
#[derive(Clone, Debug)]
pub struct FieldConfig {
    advice: [Column<Advice>; 3],
    s_is_zero: Selector,
}

impl<F: Field> FieldChip<F> {
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 3],
    ) -> <Self as Chip<F>>::Config {
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_is_zero = meta.selector();
        // b = 1 or a * c = 1
        meta.create_gate("is_inv", |meta| {
            let s = meta.query_selector(s_is_zero);
            let a = meta.query_advice(advice[0], Rotation::cur());
            let b = meta.query_advice(advice[1], Rotation::cur());
            let c = meta.query_advice(advice[2], Rotation::cur());
            vec![s * (b - Expression::Constant(F::ONE)) * (a * c - Expression::Constant(F::ONE))]
        });
        // a*b always equals 0
        meta.create_gate("mul", |meta| {
            let s = meta.query_selector(s_is_zero);
            let a = meta.query_advice(advice[0], Rotation::cur());
            let b = meta.query_advice(advice[1], Rotation::cur());
            vec![s * a * b]
        });
        FieldConfig { advice, s_is_zero }
    }
}

impl<F: Field> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

/// A variable representing a number.
#[derive(Clone)]
struct Number<F: Field>(AssignedCell<F, F>);

impl<F: Field> NumericInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    // Load private inputs into the circuit.
    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        values: &[Value<F>],
    ) -> Result<Vec<Self::Num>, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load private",
            |mut region| {
                values
                    .iter()
                    .enumerate()
                    .map(|(i, value)| {
                        region
                            .assign_advice(|| "private input", config.advice[0], i, || *value)
                            .map(Number)
                    })
                    .collect()
            },
        )
    }

    fn is_zero(
        &self,
        mut layouter: impl Layouter<F>,
        a: &[Self::Num],
        b: &[Self::Num],
    ) -> Result<Vec<Self::Num>, Error> {
        let config = self.config();
        assert_eq!(a.len(), b.len()); // check that the vectors are the same length

        layouter.assign_region(
            || "is_zero",
            |mut region: Region<'_, F>| {
                let result: Vec<Number<F>> = a
                    .iter()
                    .zip(b.iter())
                    .enumerate()
                    .map(|(_i, (a, b))| {
                        // Enable the s_is_zero selector
                        config.s_is_zero.enable(&mut region, 0).unwrap();
                        let _a_cell = region
                            .assign_advice(|| "a", config.advice[0], 0, || a.0.value().copied())
                            .unwrap();
                        let _b_cell = region
                            .assign_advice(|| "b", config.advice[1], 0, || b.0.value().copied())
                            .unwrap();
                        let a_inv: Value<Assigned<F>> = a.0.value_field().invert();
                        let c_cell = region
                            .assign_advice(|| "c", config.advice[2], 0, || a_inv.evaluate())
                            .unwrap();
                        Number(c_cell)
                    })
                    .collect();
                Ok(result)
            },
        )
    }
}

#[derive(Default)]
pub struct MyCircuit<F: Field> {
    pub a: Vec<Value<F>>,
    pub b: Vec<Value<F>>,
}

impl<F: Field> Circuit<F> for MyCircuit<F> {
    // Since we are using a single chip for everything, we can just reuse its config.
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // We create the three advice columns that FieldChip uses for I/O.
        let advice = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        FieldChip::configure(meta, advice)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config);

        // Load our private values into the circuit.
        let a = field_chip.load_private(layouter.namespace(|| "load a"), &self.a)?;
        let b = field_chip.load_private(layouter.namespace(|| "load b"), &self.b)?;

        field_chip.is_zero(layouter.namespace(|| "is_zero"), &a, &b)?;

        Ok(())
    }
}

#[test]
fn is_zero_circuit_test() {
    use halo2_proofs::dev::MockProver;
    use halo2curves::pasta::Fp;

    let k = 4;

    // good case 0 : input == 0 and output ==1
    // good case 1 : (input == 2 and output == 0)
    let a = [Fp::from(0), Fp::from(2), Fp::from(9)];
    let b = [Fp::from(1), Fp::from(0), Fp::from(0)];
    let circuit = MyCircuit {
        a: a.iter().map(|&x| Value::known(x)).collect(),
        b: b.iter().map(|&x| Value::known(x)).collect(),
    };
    let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    assert_eq!(prover.verify(), Ok(()));
}
