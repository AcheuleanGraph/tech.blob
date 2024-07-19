#![feature(test)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_miri::{NodeA, NodeB};

fn data_(data_len: usize) -> Vec<usize> {
  (0..data_len).collect()
}


fn criterion_benchmark(c: &mut Criterion) {
  let data = data_(1_000);
  c.bench_function("node_a_init", |b| b.iter(|| { let _node = NodeA::init(data.clone()).unwrap(); }));
  c.bench_function("node_b_init", |b| b.iter(|| { let _node = NodeB::init(data.clone()).unwrap(); }));

  let data = data_(500);
  let step: usize = 100;
  c.bench_function("node_a_step", |b| b.iter(|| {
    let mut node = NodeA::init(data.clone()).unwrap();
    for step in 0..step {
      node = NodeA::step(&node, step);
    }
  }));

  c.bench_function("node_b_step", |b| b.iter(|| {
    let mut node = NodeB::init(data.clone()).unwrap();
    for step in 0..step {
      node = NodeB::step(node, step);
    }
  }));

  c.bench_function("node_a_cut", |b| b.iter(|| {
    let mut node = NodeA::init(data.clone()).unwrap();
    for _ in 0..step {
      node = NodeA::cut(&mut node).unwrap().1;
    }
  }));

  c.bench_function("node_b_cut", |b| b.iter(|| {
    let mut node = NodeB::init(data.clone()).unwrap();
    for _ in 0..step {
      node = NodeB::cut(node).unwrap().1;
    }
  }));


  c.bench_function("node_a_add1", |b| b.iter(|| {
    let mut node = NodeA::init(data.clone()).unwrap();
    for step in 0..step {
      node = NodeA::add(&mut node, step, None);
    }
  }));

  c.bench_function("node_b_add1", |b| b.iter(|| {
    let mut node = NodeB::init(data.clone()).unwrap();
    for step in 0..step {
      node = NodeB::add(node, step, None);
    }
  }));


  c.bench_function("node_a_add2", |b| b.iter(|| {
    let mut node = NodeA::init(data.clone()).unwrap();
    for step in 500..1000 {
      node = NodeA::add(&mut node, step, Some(step));
    }
  }));

  c.bench_function("node_b_add2", |b| b.iter(|| {
    let mut node = NodeB::init(data.clone()).unwrap();
    for step in 500..1000 {
      node = NodeB::add(node, step, Some(step));
    }
  }));


  c.bench_function("node_a_tour", |b| b.iter(|| {
    let node = NodeA::init(data.clone()).unwrap();
    for _ in 0..step {
      let _ = NodeA::tour(&node);
    }
  }));

  c.bench_function("node_b_tour", |b| b.iter(|| {
    let node = NodeB::init(data.clone()).unwrap();
    for _ in 0..step {
      let _ = NodeB::tour(node);
    }
  }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);