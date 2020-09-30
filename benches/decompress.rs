use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use lz_string::{
    decompress_str,
};

const TEST_PHRASE: &str = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";

pub fn criterion_benchmark(c: &mut Criterion) {
    let compressed = lz_string::compress_str(TEST_PHRASE);
    c.bench_function("decompress_str", |b| {
        b.iter(|| decompress_str(black_box(&compressed)).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
