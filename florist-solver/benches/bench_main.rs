use criterion::criterion_main;
use florist_solver::florist_benches;

use complementing_a_strand_of_dna::ComplementingAStrandOfDna;
use computing_gc_content::ComputingGcContent;
use counting_dna_nucleotides::CountingDnaNucleotides;
use counting_point_mutations::CountingPointMutations;
use enumerating_gene_orders::EnumeratingGeneOrders;
use rabbits_and_recurrence_relations::RabbitsAndRecurrenceRelations;
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
    ),
    (
        rabbits_and_recurrence_relations,
        "Rabbits and Recurrence Relations",
        "rosalind_fib.txt",
        RabbitsAndRecurrenceRelations
    ),
    (
        computing_gc_content,
        "Computing GC Content",
        "rosalind_gc.txt",
        ComputingGcContent
    ),
    (
        counting_point_mutations,
        "Counting Point Mutations",
        "rosalind_hamm.txt",
        CountingPointMutations
    ),
    (
        enumerating_gene_orders,
        "Enumerating Gene Orders",
        "rosalind_perm.txt",
        EnumeratingGeneOrders
    )
}

criterion_main! {
    benches
}
