use std::cmp::Ordering;
use z3::ast::{Ast, Bool, BV};

use super::{Expression, Z3SolverContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Z3Expr<'ctx> {
    Bool(Bool<'ctx>),
    BV(BV<'ctx>),
}

impl<'ctx> From<BV<'ctx>> for Z3Expr<'ctx> {
    fn from(bv: BV<'ctx>) -> Self {
        Self::BV(bv)
    }
}

impl<'ctx> From<Bool<'ctx>> for Z3Expr<'ctx> {
    fn from(bv: Bool<'ctx>) -> Self {
        Self::Bool(bv)
    }
}

impl<'ctx> Z3Expr<'ctx> {
    pub(super) fn to_bool(self) -> Bool<'ctx> {
        match self {
            Z3Expr::Bool(b) => b,
            Z3Expr::BV(bv) => {
                let t = BV::from_u64(bv.get_ctx(), 1, bv.get_size());
                bv._eq(&t)
            }
        }
    }

    pub(super) fn to_bv(self) -> Z3Expr<'ctx> {
        match self {
            Z3Expr::Bool(b) => bool_to_bv(b).into(),
            Z3Expr::BV(_) => self,
        }
    }

    pub(super) fn as_bool(&self) -> &Bool<'ctx> {
        match self {
            Z3Expr::Bool(b) => b,
            Z3Expr::BV(_) => panic!("not a bool"),
        }
    }

    pub(super) fn as_bv(&self) -> &BV<'ctx> {
        match self {
            Z3Expr::Bool(_) => panic!("not a bv"),
            Z3Expr::BV(bv) => bv,
        }
    }
}

impl<'ctx> Expression for Z3Expr<'ctx> {
    type Context = Z3SolverContext<'ctx>;

    fn len(&self) -> u32 {
        match self {
            Z3Expr::Bool(_) => 1,
            Z3Expr::BV(bv) => bv.get_size(),
        }
    }

    fn zero_ext(&self, width: u32) -> Self {
        assert!(self.len() <= width);
        match self {
            Z3Expr::Bool(b) => {
                let ctx = b.get_ctx();
                let true_value = BV::from_u64(ctx, 1, width);
                let false_value = BV::from_u64(ctx, 0, width);
                b.ite(&true_value, &false_value).into()
            }
            Z3Expr::BV(bv) => match self.len().cmp(&width) {
                Ordering::Less => bv.zero_ext(width - self.len()).into(),
                Ordering::Equal => bv.clone().into(),
                Ordering::Greater => unreachable!(),
            },
        }
    }

    fn sign_ext(&self, width: u32) -> Self {
        assert!(self.len() <= width);
        match self.len().cmp(&width) {
            Ordering::Less => self.as_bv().sign_ext(width - self.len()).into(),
            Ordering::Equal => self.clone(),
            Ordering::Greater => unreachable!(),
        }
    }

    fn resize_unsigned(&self, width: u32) -> Self {
        match self.len().cmp(&width) {
            Ordering::Equal => self.clone(),
            Ordering::Less => self.zero_ext(width),
            Ordering::Greater => self.slice(0, width - 1),
        }
    }

    fn _eq(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        match self {
            Z3Expr::Bool(b) => b._eq(&other.as_bool()).into(),
            Z3Expr::BV(bv) => bv._eq(&other.as_bv()).into(),
        }
    }

    fn _ne(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self._eq(other).not()
    }

    fn ugt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvugt(&other.as_bv()).into()
    }

    fn ugte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvuge(&other.as_bv()).into()
    }

    fn ult(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvult(&other.as_bv()).into()
    }

    fn ulte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvule(&other.as_bv()).into()
    }

    fn sgt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvsgt(&other.as_bv()).into()
    }

    fn sgte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvsge(&other.as_bv()).into()
    }

    fn slt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvslt(&other.as_bv()).into()
    }

    fn slte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvsle(&other.as_bv()).into()
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvadd(&other.as_bv()).into()
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvsub(&other.as_bv()).into()
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvmul(&other.as_bv()).into()
    }

    fn udiv(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvudiv(&other.as_bv()).into()
    }

    fn sdiv(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvsdiv(&other.as_bv()).into()
    }

    fn urem(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvurem(&other.as_bv()).into()
    }

    fn srem(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        self.as_bv().bvsrem(&other.as_bv()).into()
    }

    fn not(&self) -> Self {
        self.clone().to_bool().not().into()
    }

    fn and(&self, other: &Self) -> Self {
        match self {
            Z3Expr::Bool(b) => Bool::and(b.get_ctx(), &[b, other.as_bool()]).into(),
            Z3Expr::BV(bv) => bv.bvand(&other.as_bv()).into(),
        }
    }

    fn or(&self, other: &Self) -> Self {
        match self {
            Z3Expr::Bool(b) => Bool::or(b.get_ctx(), &[b, other.as_bool()]).into(),
            Z3Expr::BV(bv) => bv.bvor(&other.as_bv()).into(),
        }
    }

    fn xor(&self, other: &Self) -> Self {
        match (self, other) {
            (Z3Expr::Bool(lhs), Z3Expr::Bool(rhs)) => lhs.xor(rhs).into(),
            (Z3Expr::Bool(lhs), Z3Expr::BV(rhs)) => {
                let a = z3::ast::BV::from_u64(lhs.get_ctx(), 0, rhs.get_size());
                let b = z3::ast::BV::from_u64(lhs.get_ctx(), 1, rhs.get_size());
                let lhs = lhs.ite(&b, &a);
                lhs.bvxor(rhs).into()
            }
            (Z3Expr::BV(lhs), Z3Expr::Bool(rhs)) => {
                let a = z3::ast::BV::from_u64(lhs.get_ctx(), 0, lhs.get_size());
                let b = z3::ast::BV::from_u64(lhs.get_ctx(), 1, lhs.get_size());
                let rhs = rhs.ite(&b, &a);
                lhs.bvxor(&rhs).into()
            }
            (Z3Expr::BV(lhs), Z3Expr::BV(rhs)) => lhs.bvxor(rhs).into(),
        }
    }

    fn sll(&self, other: &Self) -> Self {
        self.as_bv().bvshl(&other.as_bv()).into()
    }

    fn srl(&self, other: &Self) -> Self {
        self.as_bv().bvlshr(&other.as_bv()).into()
    }

    fn sra(&self, other: &Self) -> Self {
        self.as_bv().bvashr(&other.as_bv()).into()
    }

    fn ite(&self, then_bv: &Self, else_bv: &Self) -> Self {
        assert_eq!(self.len(), 1);
        let s = self.clone().to_bool();

        match then_bv {
            Z3Expr::Bool(_) => s.ite(then_bv.as_bool(), else_bv.as_bool()).into(),
            Z3Expr::BV(_) => s.ite(then_bv.as_bv(), else_bv.as_bv()).into(),
        }
    }

    fn concat(&self, other: &Self) -> Self {
        let s = self.clone().to_bv();
        let o = other.clone().to_bv();
        // s.concat(&o).into()
        // self.concat(&other).into()
        s.as_bv().concat(&o.as_bv()).into()
    }

    fn slice(&self, low: u32, high: u32) -> Self {
        assert!(low <= high);
        assert!(high <= self.len());
        self.as_bv().extract(high, low).into()
    }

    fn uaddo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        bool_to_bv(self.as_bv().bvadd_no_overflow(other.as_bv(), false)).into()
    }

    fn saddo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        bool_to_bv(self.as_bv().bvadd_no_overflow(other.as_bv(), true)).into()
    }

    fn usubo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        bool_to_bv(self.as_bv().bvsub_no_underflow(other.as_bv(), false)).into()
    }

    fn ssubo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        bool_to_bv(self.as_bv().bvsub_no_underflow(other.as_bv(), true)).into()
    }

    fn umulo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        bool_to_bv(self.as_bv().bvmul_no_overflow(other.as_bv(), false)).into()
    }

    fn smulo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        bool_to_bv(self.as_bv().bvmul_no_overflow(other.as_bv(), true)).into()
    }

    fn uadds(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        // self.as_bv().bvadd
        // temp
        BV::from_u64(self.as_bv().get_ctx(), 0, 1).into()
    }

    fn sadds(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        // temp
        BV::from_u64(self.as_bv().get_ctx(), 0, 1).into()
    }

    fn simplify(self) -> Self {
        match self {
            Z3Expr::Bool(b) => b.simplify().into(),
            Z3Expr::BV(bv) => bv.simplify().into(),
        }
    }

    fn get_constant(&self) -> Option<u64> {
        let e = self.clone().simplify();

        use Z3Expr::*;
        match e {
            Bool(b) => b.as_bool().map(|b| if b { 1 } else { 0 }),
            BV(bv) => bv.as_u64(),
        }
    }

    fn get_constant_bool(&self) -> Option<bool> {
        let e = self.clone().simplify();

        use Z3Expr::*;
        match e {
            Bool(b) => b.as_bool(),
            BV(bv) => bv.as_u64().map(|v| if v > 0 { true } else { false }),
        }
    }

    fn to_binary_string(&self) -> String {
        // TODO: Check if there's a better way to get the an underlying string.
        if self.len() <= 64 {
            format!("{:b}", self.get_constant().unwrap())
        } else {
            let upper = self.slice(64, self.len() - 1).to_binary_string();
            let lower = self.slice(0, 63).to_binary_string();
            format!("{}{}", upper, lower)
        }
    }

    fn get_ctx(&self) -> Self::Context {
        let ctx = match self {
            Z3Expr::Bool(b) => b.get_ctx(),
            Z3Expr::BV(bv) => bv.get_ctx(),
        };

        Z3SolverContext { ctx }
    }
}

fn bool_to_bv<'ctx>(b: Bool<'ctx>) -> BV<'ctx> {
    let zero = BV::from_u64(b.get_ctx(), 0, 1);
    let one = BV::from_u64(b.get_ctx(), 1, 1);
    b.ite(&one, &zero)
}
