use forest::{ForestContext, NodeRef};

fn normal_form<'a>(ctx: &mut ForestContext<'a>,
                   equations: Vec<NodeRef<'a>>) -> Vec<NodeRef<'a>> {
    let mut results = equations.clone();
    for (i, e) in equations.iter().enumerate() {
        results = results.into_iter().enumerate().map(|j, o| {
            if j == i { return o }
            let (lead, _) = ctx.lead_and_degree(*e);
            let remainder = ctx.divide_by_monomial(o, lead);
            if remainder == 0 { return o }
            // o = o - remainder*e
            return o
        }).collect();
    }
}
