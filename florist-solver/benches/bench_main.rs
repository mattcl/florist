use criterion::criterion_main;
use florist_solver::florist_benches;

use complementing_a_strand_of_dna::ComplementingAStrandOfDna;
use counting_dna_nucleotides::CountingDnaNucleotides;
use transcribing_dna_into_rna::TranscribingDnaIntoRna;

florist_benches! {
    10,
    (
        counting_dna_nucleotides,
        "Counting DNA Nucleotides",
        "rosalind_dna.txt",
        CountingDnaNucleotides
    ),
    (
        transcribing_dna_into_rna,
        "Transcribing DNA into RNA",
        "rosalind_rna.txt",
        TranscribingDnaIntoRna
    ),
    (
        complementing_a_strand_of_dna,
        "Complementing a Strand of DNA",
        "rosalind_revc.txt",
        ComplementingAStrandOfDna
    )
}

criterion_main! {
    benches
}
