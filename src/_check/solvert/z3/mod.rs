#![allow(dead_code)]

use z3::Context;
use z3::{ast, ast::Ast};

use super::BinaryOperation;

mod imp;

pub struct BVArray(u64);

#[derive(Clone)]
pub struct BV<'ctx>(ast::BV<'ctx>);

impl<'ctx> BV<'ctx> {
    /// Returns the BV's length in bits.
    pub fn len(&self) -> u32 {
        self.0.get_size()
    }

    pub fn zero_ext(&self, width: u32) -> BV<'ctx> {
        match self.len().cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.zero_ext(width - self.len())),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    pub fn sign_ext(&self, width: u32) -> BV<'ctx> {
        match self.len().cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.sign_ext(width - self.len())),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    pub fn binary_op(&self, other: BV<'ctx>, binop: BinaryOperation) -> Self {
        match binop {
            BinaryOperation::Add => BV(self.0.bvadd(&other.0)),
            BinaryOperation::Sub => BV(self.0.bvsub(&other.0)),
            BinaryOperation::Mul => BV(self.0.bvmul(&other.0)),
            BinaryOperation::UDiv => BV(self.0.bvudiv(&other.0)),
            BinaryOperation::SDiv => BV(self.0.bvsdiv(&other.0)),
            BinaryOperation::URem => BV(self.0.bvurem(&other.0)),
            BinaryOperation::SRem => BV(self.0.bvsrem(&other.0)),
        }
    }

    pub fn assert(&self) {
        // self.0.assert();
        todo!()
    }

    // pub fn get_solution(&self) -> BVSolution {
    //     self.0.get_a_solution()
    // }

    pub fn eq(&self, other: &BV<'ctx>) -> ast::Bool<'ctx> {
        self.0._eq(&other.0)
    }

    pub fn ne(&self, other: &BV<'ctx>) -> ast::Bool<'ctx> {
        self.eq(other).not()
    }

    pub fn add(&self, other: &BV<'ctx>) -> BV<'ctx> {
        BV(self.0.bvadd(&other.0))
    }

    pub fn sub(&self, other: &BV<'ctx>) -> BV<'ctx> {
        BV(self.0.bvsub(&other.0))
    }
}

pub struct Solver<'ctx> {
    ctx: &'ctx Context,

    solver: z3::Solver<'ctx>,
}

impl<'ctx> Solver<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        // let config = z3::Config::new();
        // let ctx = z3::Context::new(&config);
        // let ctx = std::rc::Rc::new(ctx);

        let solver = z3::Solver::new(&ctx);

        Self { ctx, solver }
    }

    /// Check if it is satisfiable.
    pub fn is_sat(&self) -> anyhow::Result<bool> {
        match self.solver.check() {
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("solver result unknown")),
        }
    }

    /// Check if sat with the extra constraint.
    pub fn is_sat_with_constraint(&self, constraint: &ast::Bool<'ctx>) -> anyhow::Result<bool> {
        // Can't i use assume here?
        // it should only be valid until the next call to is_sat

        // fn check_assumptions(&self, assumptions: &[Bool<'ctx>]) -> SatResult
        // may be of interest

        self.push();
        self.solver.assert(constraint);
        let is_sat = self.is_sat();

        self.pop();

        is_sat
    }

    pub fn get_solutions_for_bv<T>(&self, ast: &T, max_solutions: usize)
    where
        T: Ast<'ctx>,
    {
        // Setup before checking for solutions.
        self.push();
        // self.btor.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = self.internal_get_solutions_for_bv(ast, max_solutions);

        // Restore solver to initial state.
        // self.btor.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        self.pop();

        todo!()
    }

    /// Helper to ensure we always set `ModelGen::Disabled` for all paths in this function.
    fn internal_get_solutions_for_bv<T>(&self, ast: &T, max_solutions: usize)
    where
        T: Ast<'ctx>,
    {
        // if !self.is_sat()? {
        //     return Ok(Solutions::None);
        // }

        let model = self.solver.get_model().unwrap();

        let mut solutions = Vec::new();
        while solutions.len() <= max_solutions && self.is_sat().unwrap() {
            // let solution = bv.get_solution().disambiguate();

            // Constrain the next value to not be an already found solution.
            // let solution_bv = self.from_binary_string(solution.as_01x_str());
            // bv._ne(&solution_bv);

            let solution = model.eval(ast, true).unwrap();

            solutions.push(solution);
        }

        todo!()
        // Ok(match solutions.len() {
        //     0 => Solutions::None,
        //     n if n > max_solutions => Solutions::AtLeast(solutions),
        //     _ => Solutions::Exactly(solutions),
        // })
    }

    // // -------------------------------------------------------------------------
    // // Incremental stuff
    // // -------------------------------------------------------------------------

    pub fn push(&self) {
        self.solver.push()
    }
    pub fn pop(&self) {
        self.solver.pop(1)
    }

    // // -------------------------------------------------------------------------
    // // BV creation
    // // -------------------------------------------------------------------------

    pub fn array(
        &'ctx self,
        index_width: u32,
        element_width: u32,
        symbol: Option<&str>,
    ) -> ast::Array<'ctx> {
        // boolector::Array::new(self.btor.clone(), index_width, element_width, symbol)

        let domain = z3::Sort::bitvector(&self.ctx, index_width);
        let range = z3::Sort::bitvector(&self.ctx, element_width);
        ast::Array::new_const(&self.ctx, symbol.unwrap(), &domain, &range)
    }

    pub fn bv(&'ctx self, bits: u32) -> BV<'ctx> {
        // boolector::BV::new(self.btor.clone(), bits, None).into()
        todo!()
    }

    pub fn bv_from_bool(&'ctx self, value: bool) -> BV<'ctx> {
        // boolector::BV::from_bool(self.btor.clone(), value).into()
        todo!()
    }

    /// Creates new bv from a u64.
    pub fn bv_from_u64(&'ctx self, value: u64, bits: u32) -> BV<'ctx> {
        BV(ast::BV::from_u64(&self.ctx, value, bits))
    }

    /// Gets a new bv with a zero value.
    pub fn bv_zero(&'ctx self, bits: u32) -> BV<'ctx> {
        // boolector::BV::zero(self.btor.clone(), bits).into()
        Self::bv_from_u64(&self, 0, bits)
    }

    pub fn bv_from_f32(&'ctx self, value: f32) -> ast::Float<'ctx> {
        ast::Float::from_f32(&self.ctx, value)
    }

    pub fn bv_from_f64(&'ctx self, value: f64) -> ast::Float<'ctx> {
        ast::Float::from_f64(&self.ctx, value)
    }

    // pub fn from_binary_string(&'ctx self, bits: &str) -> BV<'ctx> {
    //     // boolector::BV::from_binary_str(self.btor.clone(), bits).into()
    //     todo!()
    // }
}
