use criterion::{criterion_group, criterion_main, Criterion};

use f3l_search_tree::{KdTree, OcTree, TreeSearch};
use kdtree::distance::squared_euclidean;

fn load_ply(path: &str) -> Vec<[f32; 3]> {
    use ply_rs as ply;
    use ply_rs::ply::Property;

    let mut f = std::fs::File::open(path).unwrap();
    // create a parser
    let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();
    // use the parser: read the entire file
    let ply = p.read_ply(&mut f);
    // make sure it did work
    assert!(ply.is_ok());

    let ply_wrapper = ply.unwrap();

    let vertices = ply_wrapper.payload["vertex"]
        .iter()
        .map(|v| {
            let vertex = [v["x"].clone(), v["y"].clone(), v["z"].clone()];
            vertex
                .iter()
                .map(|v| match v {
                    Property::Float(f) => *f,
                    Property::Double(d) => *d as f32,
                    _ => 0f32,
                })
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();

    vertices
        .into_iter()
        .map(|v| [v[0], v[1], v[2]])
        .collect()
}


fn bench_tree_build(c: &mut Criterion) {
    let data = load_ply("data/table_voxel_down.ply");
    let mut group = c.benchmark_group("Tree-Build");
    group.bench_function("KD-Tree", |b| {
        b.iter(|| {
            let mut tree = KdTree::with_data(&data);
            tree.build();
        })
    });
    group.bench_function("Oc-Tree_100_3", |b| {
        b.iter(|| {
            let mut tree = OcTree::with_data(&data, 100, 3);
            tree.build();
        })
    });
    group.bench_function("Oc-Tree_100_8", |b| {
        b.iter(|| {
            let mut tree = OcTree::with_data(&data, 100, 8);
            tree.build();
        })
    });
    group.bench_function("crate KDTree", |b| {
        b.iter(|| {
            let mut kdtree = kdtree::KdTree::with_capacity(3, data.len());
            (0..data.len()).for_each(|i| kdtree.add(&data[i], i).unwrap());
        })
    });
    group.bench_function("crate KD-Tree", |b| {
        b.iter(|| {
            let _tree = kd_tree::KdTree::build_by_ordered_float(data.clone());
        })
    });
    group.bench_function("crate rstart", |b| {
        b.iter(|| {
            let _tree = rstar::RTree::bulk_load(data.clone());
        })
    });
}


fn bench_tree_knn_1(c: &mut Criterion) {
    let data = load_ply("data/table_scene_lms400.ply");
    let mut group = c.benchmark_group("Tree-Search-Nearest_1");

    let target = &data[data.len() / 2];

    group.bench_function("KD", |b| {
        let mut tree = KdTree::with_data(&data);
        tree.build();
        b.iter(|| {
            tree.search_knn(target, 1);
        });
    });
    group.bench_function("Oc-Tree_100_3", |b| {
        let mut tree = OcTree::with_data(&data, 100, 3);
        tree.build();
        b.iter(|| {
            tree.search_knn(target, 1);
        });
    });
    group.bench_function("Oc-Tree_100_8", |b| {
        let mut tree = OcTree::with_data(&data, 100, 8);
        tree.build();
        b.iter(|| {
            tree.search_knn(target, 1);
        });
    });
    group.bench_function("crate KDTree", |b| {
        let mut tree = kdtree::KdTree::with_capacity(3, data.len());
        (0..data.len()).for_each(|i| tree.add(&data[i], i).unwrap());
        b.iter(|| {
            tree.nearest(target, 1, &squared_euclidean).unwrap();
        });
    });
    group.bench_function("crate KD-Tree", |b| {
        let tree = kd_tree::KdTree::build_by_ordered_float(data.clone());
        b.iter(|| {
            tree.nearest(target).unwrap();
        });
    });
    group.bench_function("crate rstart", |b| {
        let tree = rstar::RTree::bulk_load(data.clone());
        b.iter(|| {
            tree.nearest_neighbor(target).unwrap();
        })
    });
}

fn bench_tree_knn_10(c: &mut Criterion) {
    let data = load_ply("data/table_scene_lms400.ply");
    let mut group = c.benchmark_group("Tree-Search-Nearest_10");

    let target = &data[data.len() / 2];

    group.bench_function("KD", |b| {
        let mut tree = KdTree::with_data(&data);
        tree.build();
        b.iter(|| {
            tree.search_knn(target, 10);
        });
    });
    group.bench_function("Oc-Tree_100_3", |b| {
        let mut tree = OcTree::with_data(&data, 100, 3);
        tree.build();
        b.iter(|| {
            tree.search_knn(target, 10);
        });
    });
    group.bench_function("Oc-Tree_100_8", |b| {
        let mut tree = OcTree::with_data(&data, 100, 8);
        tree.build();
        b.iter(|| {
            tree.search_knn(target, 10);
        });
    });
    group.bench_function("crate KDTree", |b| {
        let mut tree = kdtree::KdTree::with_capacity(3, data.len());
        (0..data.len()).for_each(|i| tree.add(&data[i], i).unwrap());
        b.iter(|| {
            tree.nearest(target, 10, &squared_euclidean).unwrap();
        });
    });
    group.bench_function("crate KD-Tree", |b| {
        let tree = kd_tree::KdTree::build_by_ordered_float(data.clone());
        b.iter(|| {
            tree.nearests(target, 10);
        });
    });
    group.bench_function("crate rstart", |b| {
        let tree = rstar::RTree::bulk_load(data.clone());
        b.iter(|| {
            let _ = tree.nearest_neighbor_iter_with_distance_2(target).enumerate()
                .take_while(|(i, _)| *i<10).collect::<Vec<_>>();
        })
    });
}


fn bench_tree_radius_search_0_03(c: &mut Criterion) {
    let data = load_ply("data/table_scene_lms400.ply");
    let mut group = c.benchmark_group("Tree-Search-Radius_0.03");

    let target = &data[data.len() / 2];

    group.bench_function("KD", |b| {
        let mut tree = KdTree::with_data(&data);
        tree.build();
        b.iter(|| {
            tree.search_radius(target, 0.03);
        });
    });
    group.bench_function("Oc-Tree_100_3", |b| {
        let mut tree = OcTree::with_data(&data, 100, 3);
        tree.build();
        b.iter(|| {
            tree.search_radius(target, 0.03);
        });
    });
    group.bench_function("Oc-Tree_100_8", |b| {
        let mut tree = OcTree::with_data(&data, 100, 8);
        tree.build();
        b.iter(|| {
            tree.search_radius(target, 0.03);
        });
    });
    group.bench_function("crate KDTree", |b| {
        let mut tree = kdtree::KdTree::with_capacity(3, data.len());
        (0..data.len()).for_each(|i| tree.add(&data[i], i).unwrap());
        b.iter(|| {
            tree.within(target, 0.03, &squared_euclidean).unwrap();
        });
    });
    group.bench_function("crate KD-Tree", |b| {
        let tree = kd_tree::KdTree::build_by_ordered_float(data.clone());
        b.iter(|| {
            tree.within_radius(target, 0.03);
        });
    });
}

fn bench_tree_radius_search_0_08(c: &mut Criterion) {
    let data = load_ply("data/table_scene_lms400.ply");
    let mut group = c.benchmark_group("Tree-Search-Radius_0.08");

    let target = &data[data.len() / 2];

    group.bench_function("KD", |b| {
        let mut tree = KdTree::with_data(&data);
        tree.build();
        b.iter(|| {
            tree.search_radius(target, 0.08);
        });
    });
    group.bench_function("Oc-Tree_100_3", |b| {
        let mut tree = OcTree::with_data(&data, 100, 3);
        tree.build();
        b.iter(|| {
            tree.search_radius(target, 0.08);
        });
    });
    group.bench_function("Oc-Tree_100_8", |b| {
        let mut tree = OcTree::with_data(&data, 100, 8);
        tree.build();
        b.iter(|| {
            tree.search_radius(target, 0.08);
        });
    });
    group.bench_function("crate KDTree", |b| {
        let mut tree = kdtree::KdTree::with_capacity(3, data.len());
        (0..data.len()).for_each(|i| tree.add(&data[i], i).unwrap());
        b.iter(|| {
            tree.within(target, 0.08, &squared_euclidean).unwrap();
        });
    });
    group.bench_function("crate KD-Tree", |b| {
        let tree = kd_tree::KdTree::build_by_ordered_float(data.clone());
        b.iter(|| {
            tree.within_radius(target, 0.08);
        });
    });
}

criterion_group!(
    search_tree_bench,
    bench_tree_build,
    bench_tree_knn_1,
    bench_tree_knn_10,
    bench_tree_radius_search_0_03,
    bench_tree_radius_search_0_08
);

criterion_main!(search_tree_bench);

