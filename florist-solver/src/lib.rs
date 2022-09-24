#[macro_export]
macro_rules! florist_bench {
    ($name:ident, $label:literal, $file:literal, $solver:ty) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group($label);
            group.bench_function("solution", |b| {
                let raw = load_input_file($file).expect("Failed to load file");
                b.iter(|| {
                    let input = <$solver>::input_from(&raw).expect("Failed to make input");
                    <$solver>::solve(input).expect("Failed to solve");
                })
            });
            group.finish();
        }
    };
}

#[macro_export]
macro_rules! florist_benches {
    ($comb_seconds:literal, $(($name:ident, $label:literal, $file:literal, $solver:ty)),+) => {
        use criterion::{criterion_group, Criterion};
        use std::time::Duration;

        use florist_plumbing::{load_input_file, Problem};

        $(
            florist_solver::florist_bench!($name, $label, $file, $solver);
        )+

        pub fn florist_combined(c: &mut Criterion) {
            let mut group = c.benchmark_group("Rosalind Problems");
            group.measurement_time(Duration::new($comb_seconds, 0));
            group.bench_function("Total runtime for all solutions, including parsing", |b| {
                b.iter(|| {
                    $(
                        let raw = load_input_file($file).expect("Failed to load file");
                        let input = <$solver>::input_from(&raw).expect("Failed to make input");
                        <$solver>::solve(input).expect("Failed to solve");
                    )+
                })
            });
            group.finish();
        }

        criterion_group!(benches, $($name,)+ florist_combined);
    };
    ($(($name:ident, $label:literal, $file:literal, $solver:ty)),+) => {
        florist_benches!{
            10, $( ($name, $label, $file, $solver)),+
        }
    };
}

#[cfg(test)]
mod tests {
    use complementing_a_strand_of_dna::ComplementingAStrandOfDna;
    use computing_gc_content::ComputingGcContent;
    use counting_dna_nucleotides::CountingDnaNucleotides;
    use counting_point_mutations::CountingPointMutations;
    use enumerating_gene_orders::EnumeratingGeneOrders;
    use florist_plumbing::{load_input_file, Problem};
    use rabbits_and_recurrence_relations::RabbitsAndRecurrenceRelations;
    use transcribing_dna_into_rna::TranscribingDnaIntoRna;

    macro_rules! test_case {
        ($name:ident, $file:literal, $solver:ty, $expected:literal) => {
            #[test]
            fn $name() {
                let f = load_input_file($file).expect("Failed to load file");
                let input = <$solver>::input_from(&f).expect("Failed to make input");
                let output = <$solver>::solve(input).expect("Failed to solve");

                assert_eq!(output.to_string().as_str(), $expected);
            }
        };
    }

    test_case!(
        counting_dna_nucleotides,
        "rosalind_dna.txt",
        CountingDnaNucleotides,
        "205 211 214 209"
    );

    test_case!(
        transcribing_dna_into_rna,
        "rosalind_rna.txt",
        TranscribingDnaIntoRna,
        "CUAAGCAGCAAUGCUGCACCGCGUGAAGAGCAGUGUGGUAGUCCACUUCCGUGACCCAUAAUUAGAGGUGCUGCUUCACUAGCGUGUGCACCUUACAUCCCACGGACGGGUGAGGGCUACUCUGCAUAAUUUGUCAUGCAUCCUUACAUUGUUUCAGAUUGACACUAAUAAGGGCACCAACUGCCAGGCAUCGAAGAAGGCAAUACUGGCUCAGGAACGGGGUAGAGCUUGUGGAUGCGAUCAUACAAAAGUCUUAAGCCUAUCUUCAAGCCUCUUUCUUGAUGCGGAAGCGAAAGCCACUUUCGUGUCCAGCGAGCGACGAUGCUAUUAGAACCUUAGGUUGUAUUACGCAGAACUCCGAGAAUUAUGGAACCAGGUGCCUUUAGGGAUUCAGACAGGCGAGACCCGGAGUCGAACUUUCACUACGACGCUAGCGGGAGGUUGGAGCUCUCGACCAUAUAGAAGAAGAGUAGUCCCAGGGGUAAGCGGCCCGAUAAAAACUUGACUCCUGGUGCUAUGCUCGUAGUAGGACAACAAGGCCGAUGCUUGUUGUGACCCAAGAAAUCUGUCGUGAGGAUACACUGGGCGUACUUAAAAAGAAGUCUACUUCAGUUGGAGCGCCUGAGGCGGACCUACUAAGGACCUUAGGGGAUGACGUUGCAAUACACAUUUCCUCCAACGAUCACAGUUGGUGGACCAUAGCUCAAACCUGAGCGAGCUGGUAUCUAGGACCCUGACUGCACGUCUGCUGCUGAAUAUGAUCUCUACAACCGGGUGAACAGAGCUACCCUGCAUGGCCCGUGAAGAAUCGCUUGCGGCUUAAGUCAUCAAUCCCAGGAGUUUAGAACGAAGACGCGAGGUCCAGGGACCGUCCCCGCUUGAGGUUGUUGAGGACAAGCGUCGGGCAAGCCGUCGAUCUUGGAAGAGAGUG"
    );

    test_case!(
        complementing_a_strand_of_dna,
        "rosalind_revc.txt",
        ComplementingAStrandOfDna,
        "CAGGTGGCTGGAGACGAGGATTCTGAATTCCCAGGGGTGACAAGCACTCGCACTCCTATCTACCATCGAAATGAGCCGGGCTAGCTTCTTGTTTCAGAGCCGGTTACATACTACTCCATGGTACCTCTGGACGAATCGCCGTACCCCAAGTAGTTAGTTCGGGCCAGTCTACGGGTACGAACAATGCATCGTTTATCAATTTAAAGAGTCGGCAAGCCTCACCCAGTTCAATGGGGAAATTGTGACAGCGATCCCCACCGAGATCATACTGTGCGTACTTTAAGACTCTGAAGGAAAGAATGGTACACCAGTCATTTTATAGAAGCGAGTAACCACAGGGAGTACGAAACACGCTACTTTTCCAACGTCAGTGGACGATCATGGCGCGAGAAACCAGGTAGATGTAGCCTATACCAAACTGTTCTTGGTGTGTGAGCTCGCTGGGCAGGATCACGCCCGGAGTCGCCTGTCCACCCAGTAGACCGGGGGCTGTGTTGCCCCGCGCTCCAGGAGTGAACGACTAAATTTAGTGAATCTCATGCATGGCTAGCCGCGTGCCCCCGATTTTCTATAAGGGACGATTGAGGTAAGTTTCGTTGACTTGAGTAGTTCGACCTCATATAATGAGATTACCTGTTCGCACAGCAATCTTGTGAGCAGAGGAACCCCTACCTGGCAGAAACTAGCAAGATCGCAACTACTATAACTATGAGATCGGTAAGAAAAACTGTTAATCGGGGATGGCCGACGTTGTACCGGAACATCATCCGATAGCCTAAGGTTGATGCCAGCCATCCGGGCTGCGAACGGTATATGAGTACACACTCATTAGCGTTACTCCCCCCAAGATCCCGCCACCGGAACGCCCGCCCGGACACAAGGTCCTCGGTAGTGGGGACAATCCCGTGGAGCCTTTGATAGCGTAAGTACGAGCAGTGATATCTTTGCCTAAAAG"
    );

    test_case!(
        rabbits_and_recurrence_relations,
        "rosalind_fib.txt",
        RabbitsAndRecurrenceRelations,
        "574888488199"
    );

    test_case!(
        computing_gc_content,
        "rosalind_gc.txt",
        ComputingGcContent,
        "Rosalind_9913\n51.2396694214876"
    );

    test_case!(
        counting_point_mutations,
        "rosalind_hamm.txt",
        CountingPointMutations,
        "491"
    );

    test_case!(
        enumerating_gene_orders,
        "rosalind_perm.txt",
        EnumeratingGeneOrders,
        "120\n1 2 3 4 5\n1 2 3 5 4\n1 2 4 3 5\n1 2 4 5 3\n1 2 5 3 4\n1 2 5 4 3\n1 3 2 4 5\n1 3 2 5 4\n1 3 4 2 5\n1 3 4 5 2\n1 3 5 2 4\n1 3 5 4 2\n1 4 2 3 5\n1 4 2 5 3\n1 4 3 2 5\n1 4 3 5 2\n1 4 5 2 3\n1 4 5 3 2\n1 5 2 3 4\n1 5 2 4 3\n1 5 3 2 4\n1 5 3 4 2\n1 5 4 2 3\n1 5 4 3 2\n2 1 3 4 5\n2 1 3 5 4\n2 1 4 3 5\n2 1 4 5 3\n2 1 5 3 4\n2 1 5 4 3\n2 3 1 4 5\n2 3 1 5 4\n2 3 4 1 5\n2 3 4 5 1\n2 3 5 1 4\n2 3 5 4 1\n2 4 1 3 5\n2 4 1 5 3\n2 4 3 1 5\n2 4 3 5 1\n2 4 5 1 3\n2 4 5 3 1\n2 5 1 3 4\n2 5 1 4 3\n2 5 3 1 4\n2 5 3 4 1\n2 5 4 1 3\n2 5 4 3 1\n3 1 2 4 5\n3 1 2 5 4\n3 1 4 2 5\n3 1 4 5 2\n3 1 5 2 4\n3 1 5 4 2\n3 2 1 4 5\n3 2 1 5 4\n3 2 4 1 5\n3 2 4 5 1\n3 2 5 1 4\n3 2 5 4 1\n3 4 1 2 5\n3 4 1 5 2\n3 4 2 1 5\n3 4 2 5 1\n3 4 5 1 2\n3 4 5 2 1\n3 5 1 2 4\n3 5 1 4 2\n3 5 2 1 4\n3 5 2 4 1\n3 5 4 1 2\n3 5 4 2 1\n4 1 2 3 5\n4 1 2 5 3\n4 1 3 2 5\n4 1 3 5 2\n4 1 5 2 3\n4 1 5 3 2\n4 2 1 3 5\n4 2 1 5 3\n4 2 3 1 5\n4 2 3 5 1\n4 2 5 1 3\n4 2 5 3 1\n4 3 1 2 5\n4 3 1 5 2\n4 3 2 1 5\n4 3 2 5 1\n4 3 5 1 2\n4 3 5 2 1\n4 5 1 2 3\n4 5 1 3 2\n4 5 2 1 3\n4 5 2 3 1\n4 5 3 1 2\n4 5 3 2 1\n5 1 2 3 4\n5 1 2 4 3\n5 1 3 2 4\n5 1 3 4 2\n5 1 4 2 3\n5 1 4 3 2\n5 2 1 3 4\n5 2 1 4 3\n5 2 3 1 4\n5 2 3 4 1\n5 2 4 1 3\n5 2 4 3 1\n5 3 1 2 4\n5 3 1 4 2\n5 3 2 1 4\n5 3 2 4 1\n5 3 4 1 2\n5 3 4 2 1\n5 4 1 2 3\n5 4 1 3 2\n5 4 2 1 3\n5 4 2 3 1\n5 4 3 1 2\n5 4 3 2 1"
    );
}
