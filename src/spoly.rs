use super::forest::{Forest, NodeIdx};
use super::add::add;
use super::multiply::multiply;
use super::divide::divide;
use super::lead::lead;
use super::{minmax, Cache};

pub fn spoly(c: &mut Cache,
             f: &mut Forest,
             lhs: NodeIdx,
             rhs: NodeIdx) -> NodeIdx
{
    let (lhs, rhs) = minmax(lhs, rhs);

    if let Some(result) = c.spoly.get(&(lhs, rhs)) {
        return result
    }

    let lead_lhs = lead(c, f, lhs, None);
    let lead_rhs = lead(c, f, rhs, None);

    let prod = multiply(c, f, lead_lhs, lead_rhs);
    let prod_div_lhs = divide(c, f, prod, lead_lhs);
    let prod_div_rhs = divide(c, f, prod, lead_rhs);

    let lhs_adj = multiply(c, f, lhs, prod_div_lhs);
    let rhs_adj = multiply(c, f, rhs, prod_div_rhs);

    let result = add(c, f, lhs_adj, rhs_adj);

    c.spoly.set((lhs, rhs), result)
}
