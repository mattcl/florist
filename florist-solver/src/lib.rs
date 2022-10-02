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
    use std::path::Path;

    use calculating_expected_offspring::CalculatingExpectedOffspring;
    use calculating_protein_mass::CalculatingProteinMass;
    use complementing_a_strand_of_dna::ComplementingAStrandOfDna;
    use computing_gc_content::ComputingGcContent;
    use consensus_and_profile::ConsensusAndProfile;
    use counting_dna_nucleotides::CountingDnaNucleotides;
    use counting_point_mutations::CountingPointMutations;
    use enumerating_gene_orders::EnumeratingGeneOrders;
    use enumerating_k_mers_lexicographically::EnumeratingKMersLexicographically;
    use finding_a_motif_in_dna::FindingAMotifInDna;
    use florist_plumbing::{load_input_file, Problem};
    use independent_alleles::IndependentAlleles;
    use inferring_mrna_from_protein::InferringMrnaFromProtein;
    use locating_restriction_sites::LocatingRestrictionSites;
    use mendels_first_law::MendelsFirstLaw;
    use mortal_fibonacci_rabbits::MortalFibonacciRabbits;
    use open_reading_frames::OpenReadingFrames;
    use overlap_graphs::SortedOverlapGraphs;
    use rabbits_and_recurrence_relations::RabbitsAndRecurrenceRelations;
    use rna_splicing::RnaSplicing;
    use transcribing_dna_into_rna::TranscribingDnaIntoRna;
    use translating_rna_into_protein::TranslatingRnaIntoProtein;

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

    fn load_solution(file: &str) -> String {
        let p = Path::new("solutions").join(file);
        std::fs::read_to_string(p).expect("Failed to load solution file")
    }

    macro_rules! test_case_extern_solution {
        ($name:ident, $file:literal, $solver:ty, $solution:literal) => {
            #[test]
            fn $name() {
                let f = load_input_file($file).expect("Failed to load file");
                let input = <$solver>::input_from(&f).expect("Failed to make input");
                let output = <$solver>::solve(input).expect("Failed to solve");

                assert_eq!(output.to_string().as_str(), load_solution($solution).trim());
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

    test_case_extern_solution!(
        enumerating_gene_orders,
        "rosalind_perm.txt",
        EnumeratingGeneOrders,
        "enumerating_gene_orders.txt"
    );

    test_case_extern_solution!(
        enumerating_k_mers_lexicographically,
        "rosalind_lexf.txt",
        EnumeratingKMersLexicographically,
        "enumerating_k_mers_lexicographically.txt"
    );

    test_case!(
        mendels_first_law,
        "rosalind_iprb.txt",
        MendelsFirstLaw,
        "0.7584388185654007"
    );

    test_case_extern_solution!(
        translating_rna_into_protein,
        "rosalind_prot.txt",
        TranslatingRnaIntoProtein,
        "translating_rna_into_protein.txt"
    );

    test_case!(
        calculating_protein_mass,
        "rosalind_prtm.txt",
        CalculatingProteinMass,
        "117772.67882000057"
    );

    test_case!(
        finding_a_motif_in_dna,
        "rosalind_subs.txt",
        FindingAMotifInDna,
        "89 104 111 140 176 227 255 262 269 301 359 370 397 428 456 475 510 601 608 637 644 651 658 694 747 804"
    );

    test_case!(
        calculating_expected_offspring,
        "rosalind_iev.txt",
        CalculatingExpectedOffspring,
        "147988"
    );

    test_case_extern_solution!(
        locating_restriction_sites,
        "rosalind_revp.txt",
        LocatingRestrictionSites,
        "locating_restriction_sites.txt"
    );

    // Man, the authors didn't even mention that they could give inputs that
    // would overflow 64 bit unsigned integers. That's actually pretty awful,
    // considering the problem came out almost a decade ago. This would explain
    // why I got this "wrong" several times.
    test_case!(
        mortal_fibonacci_rabbits,
        "rosalind_fibd.txt",
        MortalFibonacciRabbits,
        "258314806822396236"
    );

    test_case!(
        inferring_mrna_from_protein,
        "rosalind_mrna.txt",
        InferringMrnaFromProtein,
        "549056"
    );

    test_case!(
        independent_alleles,
        "rosalind_lia.txt",
        IndependentAlleles,
        "0.30060220579105046"
    );

    test_case_extern_solution!(
        overlap_graphs,
        "rosalind_grph.txt",
        SortedOverlapGraphs,
        "overlap_graphs.txt"
    );

    test_case_extern_solution!(
        consensus_and_profile,
        "rosalind_cons.txt",
        ConsensusAndProfile,
        "consensus_and_profile.txt"
    );

    test_case_extern_solution!(
        open_reading_frames,
        "rosalind_orf.txt",
        OpenReadingFrames,
        "open_reading_frames.txt"
    );

    test_case!(
        rna_splicing,
        "rosalind_splc.txt",
        RnaSplicing,
        "MRAHASYATLLSYTALRRTFMLMKCYFGGSNRRRPREDRSFYTRPIGMPEQSRKLDGTGSGRCLSYYRKCDSPPSSPQNVEINHRGSVHIIVVSLGEEIQVSTEPDSVPKRNTDTKRPYAHECSLPEKPGWSYELEVLYEPSYFLGSTSKGLFIREATVPAQIGGLPKQHCKAVGIGRLTHAITQGESTFP"
    );
}
