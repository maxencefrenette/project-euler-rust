use float_ord::FloatOrd;
use gcd::Gcd;
use itertools::Itertools;
use std::iter;

fn slope(triple: (i32, i32, u32)) -> f32 {
    (triple.1 as f32) / (triple.0 as f32)
}

fn add(a: (i32, i32, u32), b: (i32, i32, u32)) -> (i32, i32, u32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

/// Still gives the wrong number of polygons. Might revisit.
pub fn solve() -> u64 {
    let max_p = 16;
    println!("max_p: {}", max_p);

    let mut triples: Vec<(i32, i32, u32)> = (1..max_p)
        .cartesian_product(1..max_p)
        .filter_map(|(a, b)| {
            let c = (a as f32).hypot(b as f32);

            if c.fract() < 0.00001 && a.gcd(b) == 1 {
                Some((a as i32, b as i32, c as u32))
            } else {
                None
            }
        })
        .flat_map(|(a, b, c)| {
            let pos = iter::once((a, b, c));
            let neg = iter::once((a, -b, c));

            pos.chain(neg)
        })
        .collect();

    triples.sort_by_key(|t| FloatOrd(-slope(*t)));

    println!("# of triples {:?}", triples.len());
    // println!("{:?}", triples);

    let mut half_polygons: Vec<(i32, i32, u32)> = vec![(0, 0, 0)];

    for t in triples.into_iter() {
        let mut i = 0;
        while i < half_polygons.len() {
            let candidate = add(t, half_polygons[i]);
            if candidate.2 <= max_p {
                half_polygons.push(candidate);
            }

            i += 1;
        }
    }

    half_polygons.sort();

    println!("# of half polys: {:?}", half_polygons.len());
    // println!(
    //     "{:?}",
    //     half_polygons
    //         .iter()
    //         .cloned()
    //         .take(100)
    //         .collect::<Vec<(i32, i32, u32)>>()
    // );

    let mut groups: Vec<(i32, i32, u32, u32)> = Vec::new(); // (a, b, perim, count)
    let mut i = 0;
    while i < half_polygons.len() {
        let hp = half_polygons[i];
        let mut count = 0;

        while i < half_polygons.len() && half_polygons[i] == hp {
            count += 1;
            i += 1;
        }

        groups.push((hp.0, hp.1, hp.2, count));
    }

    groups.sort_by_key(|g| g.2);

    println!("# of distinct half polys: {:?}", groups.len());

    let mut total_polys: u32 = 0;
    for (a1, b1, c1, count1) in groups.iter().cloned() {
        let mut i = 0;
        while i < groups.len() && c1 + groups[i].2 <= max_p {
            let (a2, b2, c2, count2) = groups[i];

            let mut adj_a1 = a1;
            let mut adj_b1 = b1;
            let mut adj_c1 = c1;
            let mut adj_a2 = a2;
            let mut adj_b2 = b2;
            let mut adj_c2 = c2;

            let h_diff = (adj_a1 - adj_a2).abs() as u32;
            if adj_a1.abs() < adj_a2.abs() {
                adj_a1 = adj_a2;
                adj_c1 += h_diff;
            } else if adj_a1.abs() > adj_a2.abs() {
                adj_a2 = adj_a1;
                adj_c2 += h_diff;
            }

            let v_diff = (adj_b1 - adj_b2).abs() as u32;
            if adj_b1.abs() < adj_b2.abs() {
                adj_b1 = adj_b2;
                adj_c1 += v_diff;
            } else if adj_b1.abs() > adj_b2.abs() {
                adj_b2 = adj_b1;
                adj_c2 += v_diff;
            }

            if adj_c1 == 0 && adj_c2 == 0 {
                println!("This one below");
                adj_a1 += 1;
                adj_b1 += 1;
                adj_c1 += 2;

                adj_a2 += 1;
                adj_b2 += 1;
                adj_c2 += 2;
            }

            let leftover_perim: i32 = (max_p as i32) - (adj_c1 as i32) - (adj_c2 as i32);

            if leftover_perim >= 0 {
                let n = leftover_perim as u32 / 2;
                let mut configurations = (n + 1) * (n + 2) / 2;

                let is_degen = (adj_a1 * adj_a1 + adj_b1 * adj_b1) as u32 == (adj_c1 * adj_c1)
                    && (adj_a2 * adj_a2 + adj_b2 * adj_b2) as u32 == (adj_c2 * adj_c2);
                if is_degen {
                    configurations -= 1;
                }

                println!(
                    "{:3?}, {:3?}, {:3}, {} = {}",
                    (adj_a1, adj_b1, adj_c1, count1),
                    (adj_a2, adj_b2, adj_c2, count2),
                    leftover_perim,
                    if is_degen { "degen" } else { "     " },
                    configurations
                );

                total_polys += count1 * count2 * configurations;
            }

            i += 1;
        }
    }

    // // Iterate over clusters of compatible half_polys
    // let mut i = 0;
    // let mut total_polys: u64 = 0;
    // while i < half_polygons.len() {
    //     let (a, b, _) = half_polygons[i];
    //     let mut group: Vec<(u32, bool, u32)> = Vec::new(); // (length, single, count)

    //     // println!("i = {}: ", i);

    //     while i < half_polygons.len() && half_polygons[i].0 == a && half_polygons[i].1 == b {
    //         let hp = half_polygons[i];
    //         let single = (hp.0 * hp.0 + hp.1 * hp.1) as u32 == hp.2 * hp.2;
    //         let mut count = 0;

    //         while i < half_polygons.len() && half_polygons[i] == hp {
    //             count += 1;
    //             i += 1;
    //         }

    //         group.push((hp.2, single, count));
    //     }

    //     // println!("{:?}", group);

    //     let polys: u32 = group
    //         .iter()
    //         .cloned()
    //         .cartesian_product(group.iter().cloned())
    //         .map(|((p1, single1, count1), (p2, single2, count2))| {
    //             if !(single1 && single2) && p1 + p2 <= max_p {
    //                 count1 * count2
    //             } else {
    //                 0
    //             }
    //         })
    //         .sum();

    //     // println!("{}", polys);
    //     // println!();

    //     total_polys += u64::from(polys);
    // }

    u64::from(total_polys)
}
