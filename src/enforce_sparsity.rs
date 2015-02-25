use super::forest::{Forest, Node, NodeIdx};
use super::add::add;
use super::degree::degree;
use super::Cache;

pub fn enforce_sparsity(c: &mut Cache,
                        f: &mut Forest,
                        idx: NodeIdx,
                        sparsity: usize) -> NodeIdx
{
    if idx < 2 { return idx }
    if degree(c, f, idx, None) <= sparsity { return idx }

    let Node(var, hi, lo) = f.to_node(idx);

    if sparsity == 0 { return enforce_sparsity(c, f, lo, sparsity) }

    let hi = enforce_sparsity(c, f, hi, sparsity - 1);
    let lo = enforce_sparsity(c, f, lo, sparsity);

    if hi == 0 && lo == 0 { return 0 }

    f.to_node_idx(Node(var, hi, lo))
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::forest::{Forest, Node};
    use super::super::Cache;
    use super::super::add::add;
    use super::super::multiply::multiply;

    #[test]
    fn enforce_sparsity_basic() {
        let f = &mut Forest::new();
        let c = &mut Cache::new();

        let x = f.to_node_idx(Node(0, 1, 0));
        let y = f.to_node_idx(Node(1, 1, 0));
        let z = f.to_node_idx(Node(2, 1, 0));

        let x_add_y = add(c, f, x, y);
        let x_add_z = add(c, f, x, z);
        let y_add_x = add(c, f, y, x);
        let yx_add_x = multiply(c, f, y_add_x, x);
        let yz_add_xz = multiply(c, f, y_add_x, z);
        let yz_add_xz_add_x = add(c, f, yz_add_xz, x);
        let yz_add_xz_add_x_add_z = add(c, f, yz_add_xz_add_x, z);

        assert_eq!(x_add_y, y_add_x);
        assert_eq!(enforce_sparsity(c, f, x_add_y, 1), y_add_x);
        assert_eq!(enforce_sparsity(c, f, x_add_y, 0), 0);
        assert_eq!(enforce_sparsity(c, f, yx_add_x,  1), x);
        assert_eq!(enforce_sparsity(c, f, yx_add_x,  2), yx_add_x);
        assert_eq!(enforce_sparsity(c, f, yz_add_xz_add_x,  1), x);
        assert_eq!(enforce_sparsity(c, f, yz_add_xz_add_x,  2), yz_add_xz_add_x);
        assert_eq!(enforce_sparsity(c, f, yz_add_xz_add_x_add_z,  2), yz_add_xz_add_x_add_z);
        assert_eq!(enforce_sparsity(c, f, yz_add_xz_add_x_add_z,  1), x_add_z);

    }
}
