/*
 * max_value(s, α, β)
 *   if terminal(s) return U(s)
 *   v = -∞
 *   for c in next_states(s)
 *     v' = min_value(c, α, β)
 *     if v' > v, v = v'
 *     if v' ≥ β, return v
 *     if v' > α, α = v'
 *   return v
 *
 * min_value(s, α, β)
 *   if terminal(s) return U(s)
 *   v = ∞
 *   for c in next_states(s)
 *     v' = max_value(c, α, β)
 *     if v' < v, v = v'
 *     if v' ≤ α, return v
 *     if v' < β, β = v'
 *   return v
 */

fn main() {
    println!("Hello, world!");
}
