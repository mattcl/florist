use std::{marker::PhantomData, path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells::Zsh};
use florist_plumbing::Problem;

#[derive(Parser)]
#[clap(name = "florist", version, max_term_width = 120)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn run() -> Result<()> {
        let command = Self::parse().command;
        command.run()
    }
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    GenerateCompletions(GenerateCompletions),
    CountingDnaNucleotides(Solver<counting_dna_nucleotides::CountingDnaNucleotides>),
    TranscribingDnaIntoRna(Solver<transcribing_dna_into_rna::TranscribingDnaIntoRna>),
    ComplementingAStrandOfDna(Solver<complementing_a_strand_of_dna::ComplementingAStrandOfDna>),
    RabbitsAndRecurrenceRelations(
        Solver<rabbits_and_recurrence_relations::RabbitsAndRecurrenceRelations>,
    ),
    ComputingGcContent(Solver<computing_gc_content::ComputingGcContent>),
    CountingPointMutations(Solver<counting_point_mutations::CountingPointMutations>),
    EnumeratingGeneOrders(Solver<enumerating_gene_orders::EnumeratingGeneOrders>),
    EnumeratingKMersLexicographically(
        Solver<enumerating_k_mers_lexicographically::EnumeratingKMersLexicographically>,
    ),
    MendelsFirstLaw(Solver<mendels_first_law::MendelsFirstLaw>),
    TranslatingRnaIntoProtein(Solver<translating_rna_into_protein::TranslatingRnaIntoProtein>),
    FindingAMotifInDna(Solver<finding_a_motif_in_dna::FindingAMotifInDna>),
    CalculatingExpectedOffspring(
        Solver<calculating_expected_offspring::CalculatingExpectedOffspring>,
    ),
    LocatingRestrictionSites(Solver<locating_restriction_sites::LocatingRestrictionSites>),
    MortalFibonacciRabbits(Solver<mortal_fibonacci_rabbits::MortalFibonacciRabbits>),
    InferringMrnaFromProtein(Solver<inferring_mrna_from_protein::InferringMrnaFromProtein>),
    IndependentAlleles(Solver<independent_alleles::IndependentAlleles>),
    OverlapGraphs(Solver<overlap_graphs::OverlapGraphs>),
    ConsensusAndProfile(Solver<consensus_and_profile::ConsensusAndProfile>),
    OpenReadingFrames(Solver<open_reading_frames::OpenReadingFrames>),
    RnaSplicing(Solver<rna_splicing::RnaSplicing>),
    TransitionsAndTransversions(Solver<transitions_and_transversions::TransitionsAndTransversions>),
    EnumeratingOrientedGeneOrderings(
        Solver<enumerating_oriented_gene_orderings::EnumeratingOrientedGeneOrderings>,
    ),
}

impl Commands {
    fn run(&self) -> Result<()> {
        match self {
            Self::GenerateCompletions(cmd) => cmd.run(),
            Self::CountingDnaNucleotides(cmd) => cmd.run(),
            Self::TranscribingDnaIntoRna(cmd) => cmd.run(),
            Self::ComplementingAStrandOfDna(cmd) => cmd.run(),
            Self::RabbitsAndRecurrenceRelations(cmd) => cmd.run(),
            Self::ComputingGcContent(cmd) => cmd.run(),
            Self::CountingPointMutations(cmd) => cmd.run(),
            Self::EnumeratingGeneOrders(cmd) => cmd.run(),
            Self::EnumeratingKMersLexicographically(cmd) => cmd.run(),
            Self::MendelsFirstLaw(cmd) => cmd.run(),
            Self::TranslatingRnaIntoProtein(cmd) => cmd.run(),
            Self::FindingAMotifInDna(cmd) => cmd.run(),
            Self::CalculatingExpectedOffspring(cmd) => cmd.run(),
            Self::LocatingRestrictionSites(cmd) => cmd.run(),
            Self::MortalFibonacciRabbits(cmd) => cmd.run(),
            Self::InferringMrnaFromProtein(cmd) => cmd.run(),
            Self::IndependentAlleles(cmd) => cmd.run(),
            Self::OverlapGraphs(cmd) => cmd.run(),
            Self::ConsensusAndProfile(cmd) => cmd.run(),
            Self::OpenReadingFrames(cmd) => cmd.run(),
            Self::RnaSplicing(cmd) => cmd.run(),
            Self::TransitionsAndTransversions(cmd) => cmd.run(),
            Self::EnumeratingOrientedGeneOrderings(cmd) => cmd.run(),
        }
    }
}

#[derive(Args)]
pub(crate) struct Solver<T> {
    /// The path to the input dataset
    input: PathBuf,

    #[clap(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Solver<T>
where
    T: Problem,
    <T as Problem>::Error: Into<anyhow::Error>,
    <<T as Problem>::Input as FromStr>::Err: Into<anyhow::Error>,
{
    pub fn run(&self) -> Result<()> {
        let input_raw = std::fs::read_to_string(&self.input).context("Failed to read input")?;
        // yeah, this is nuts, maybe revisit what these bounds have to actually be
        let input = T::input_from(input_raw.trim())
            .map_err(Into::<anyhow::Error>::into)
            .context("Failed to parse input")?;
        let output = T::solve(input)
            .map_err(Into::<anyhow::Error>::into)
            .context("Failed to solve")?;
        println!("{}", output);
        Ok(())
    }
}

/// Generate zsh completions
#[derive(Debug, Args)]
pub struct GenerateCompletions;

impl GenerateCompletions {
    fn run(&self) -> Result<()> {
        generate(Zsh, &mut Cli::command(), "florist", &mut std::io::stdout());
        Ok(())
    }
}
