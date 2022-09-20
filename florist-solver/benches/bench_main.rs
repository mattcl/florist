use criterion::criterion_main;
use florist_solver::florist_benches;

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
    )
}

criterion_main! {
    benches
}
