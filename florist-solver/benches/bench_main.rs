use criterion::criterion_main;
use florist_solver::florist_benches;

use calculating_expected_offspring::CalculatingExpectedOffspring;
use calculating_protein_mass::CalculatingProteinMass;
use complementing_a_strand_of_dna::ComplementingAStrandOfDna;
use computing_gc_content::ComputingGcContent;
use counting_dna_nucleotides::CountingDnaNucleotides;
use counting_point_mutations::CountingPointMutations;
use enumerating_gene_orders::EnumeratingGeneOrders;
use enumerating_k_mers_lexicographically::EnumeratingKMersLexicographically;
use finding_a_motif_in_dna::FindingAMotifInDna;
use mendels_first_law::MendelsFirstLaw;
use rabbits_and_recurrence_relations::RabbitsAndRecurrenceRelations;
use transcribing_dna_into_rna::TranscribingDnaIntoRna;
use translating_rna_into_protein::TranslatingRnaIntoProtein;

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
    ),
    (
        enumerating_k_mers_lexicographically,
        "Enumerating k-mers Lexicographically",
        "rosalind_lexf.txt",
        EnumeratingKMersLexicographically
    ),
    (
        mendels_first_law,
        "Mendel's First Law",
        "rosalind_iprb.txt",
        MendelsFirstLaw
    ),
    (
        translating_rna_into_protein,
        "Translating RNA into Protein",
        "rosalind_prot.txt",
        TranslatingRnaIntoProtein
    ),
    (
        calculating_protein_mass,
        "Calculating Protein Mass",
        "rosalind_prtm.txt",
        CalculatingProteinMass
    ),
    (
        finding_a_motif_in_dna,
        "Finding a Motif in DNA",
        "rosalind_subs.txt",
        FindingAMotifInDna
    ),
    (
        calculating_expected_offspring,
        "Calculating Expected Offspring",
        "rosalind_iev.txt",
        CalculatingExpectedOffspring
    )
}

criterion_main! {
    benches
}
