use std::{marker::PhantomData, path::PathBuf, str::FromStr};

use anyhow::{Result, Context};
use clap::{Parser, CommandFactory, Subcommand, Args};
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
    RabbitsAndRecurrenceRelations(Solver<rabbits_and_recurrence_relations::RabbitsAndRecurrenceRelations>),
    ComputingGcContent(Solver<computing_gc_content::ComputingGcContent>),
    CountingPointMutations(Solver<counting_point_mutations::CountingPointMutations>),
    EnumeratingGeneOrders(Solver<enumerating_gene_orders::EnumeratingGeneOrders>),
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
        }
    }
}

#[derive(Args)]
pub(crate) struct Solver<T> {
    /// The path to the input dataset
    input: PathBuf,

    #[clap(skip)]
    _phantom: PhantomData<T>
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
        let input = T::input_from(input_raw.trim()).map_err(Into::<anyhow::Error>::into).context("Failed to parse input")?;
        let output = T::solve(input).map_err(Into::<anyhow::Error>::into).context("Failed to solve")?;
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
