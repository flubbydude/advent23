use num::PrimInt;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap, hash::Hash};

struct StateInfo<T: Clone + PartialEq + Eq + Hash, C: PrimInt> {
    cost: C,
    parent: Option<T>,
}

// get the cost and the path taken
pub fn a_star_search<T, C>(
    initial_states: &[T],

    // boxed slice of (cost to get to successor, successor)
    get_successors: impl Fn(&T) -> Box<[(C, T)]>,

    is_goal: impl Fn(&T) -> bool,
    heuristic: impl Fn(&T) -> C,
) -> Option<(C, Vec<T>)>
where
    T: Clone + PartialEq + Eq + Hash,
    C: PrimInt,
{
    let mut frontier = initial_states
        .iter()
        .cloned()
        .map(|state| {
            let h = heuristic(&state);
            (state, Reverse(h))
        })
        .collect::<PriorityQueue<_, _>>();

    let mut state_infos = initial_states
        .iter()
        .cloned()
        .map(|state| {
            (
                state,
                StateInfo::<T, C> {
                    cost: C::zero(),
                    parent: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    while let Some((state, _)) = frontier.pop() {
        let state_cost = state_infos[&state].cost;

        if is_goal(&state) {
            let mut path = vec![state];
            while let Some(parent) = &state_infos[path.last().unwrap()].parent {
                path.push(parent.clone());
            }

            path.reverse();

            return Some((state_cost, path));
        }

        for &(state_to_succ_cost, ref successor) in get_successors(&state).iter() {
            let tentative_cost = state_cost + state_to_succ_cost;

            if let Some(succ_info) = state_infos.get_mut(successor) {
                // if tentative cost was too large
                if succ_info.cost <= tentative_cost {
                    continue;
                }

                // update the cost stored in the hash map!
                succ_info.cost = tentative_cost;
                succ_info.parent = Some(state.clone());

                // update the priority using the heuristic!
                frontier.change_priority(successor, Reverse(tentative_cost + heuristic(successor)));
            } else {
                state_infos.insert(
                    successor.clone(),
                    StateInfo {
                        cost: tentative_cost,
                        parent: Some(state.clone()),
                    },
                );
                frontier.push(
                    successor.clone(),
                    Reverse(tentative_cost + heuristic(successor)),
                );
            }
        }
    }

    None
}
