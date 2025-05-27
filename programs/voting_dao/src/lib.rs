use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GebXFPNYCQ8Gz1JcAT7BXxxD2Y6gtmzKPJVHu9WsyoKQ");

#[program]
mod vote {
    use super::*;

    /// Fonction to create a new proposal
    /// Creates a new proposal with the given title, description, choices, start date, and end date.
    /// The proposal must have between 2 and 5 choices, and the start date must be before the end date.
    /// # Arguments
    /// * `ctx` - The context containing the accounts required for the proposal.
    /// * `title` - The title of the proposal.
    /// * `description` - A brief description of the proposal.
    /// * `choices` - A vector of choices for the proposal, must contain between 2 and 5 choices.
    /// * `date_start` - The start date of the proposal in Unix timestamp format.
    /// * `date_end` - The end date of the proposal in Unix timestamp format.
    ///
    /// # Returns
    /// * `Ok(())` if the proposal is created successfully.
    /// * An error if the proposal creation fails due to invalid parameters.
    ///
    /// # Errors
    /// * `ProposalError::InvalidNumberOfChoices` if the number of choices is not between 2 and 5.
    /// * `ProposalError::DateNotConform` if the start date is not before the end date.
    ///
    pub fn create_proposal(
        ctx: Context<InitializeProposal>,
        title: String,
        description: String,
        choices: Vec<String>,
        date_start: u64,
        date_end: u64,
    ) -> Result<()> {
        require!(
            choices.len() >= 2 && choices.len() <= 5,
            ProposalError::InvalidNumberOfChoices
        );

        require!(date_start <= date_end, ProposalError::DateNotConform);

        let new_proposal = &mut ctx.accounts.proposal;

        new_proposal.creator = ctx.accounts.signer.key();
        new_proposal.title = title;
        new_proposal.description = description;
        new_proposal.date_start = date_start;
        new_proposal.date_end = date_end;

        new_proposal.votes = choices
            .into_iter()
            .map(|name| Choice {
                name,
                count: 0,
            }).collect();

        msg!("VotingApp initialized by: {}", new_proposal.creator);
        msg!("Proposal address: {}", ctx.accounts.proposal.key());

        Ok(())
    }

    /// Fonction to cast a vote for a proposal
    /// Casts a vote for a specific choice in a proposal.
    /// # Arguments
    /// * `ctx` - The context containing the accounts required for voting.
    /// * `target` - The name of the choice to vote for.
    /// # Returns
    /// * `Ok(())` if the vote is cast successfully.
    /// * An error if the vote cannot be cast due to the proposal being closed or the choice being invalid.
    /// # Errors
    /// * `ProposalError::VoteNotOpen` if the proposal is not open for voting.
    /// * `ProposalError::VoteClosed` if the proposal is closed for voting.
    /// * `ProposalError::InvalidChoice` if the choice does not exist in the proposal.
    ///
    /// # Note
    /// This function checks the current time against the proposal's start and end dates to determine if voting is allowed.
    /// It also checks if the choice exists in the proposal's list of choices.
    /// If the choice is valid, it increments the vote count for that choice.
    ///
    pub fn cast_vote(ctx: Context<InitializeVote>, target: String) -> Result<()> {
        let clock = &ctx.accounts.clock;
        let timestamp = clock.unix_timestamp as u64;
        let proposal = &mut ctx.accounts.proposal;

        require!(proposal.date_start <= timestamp, ProposalError::VoteNotOpen);
        require!(proposal.date_end > timestamp, ProposalError::VoteClosed);

        let choice = proposal.votes.iter_mut().find(|x| x.name == target);
        require!(choice.is_some(), ProposalError::InvalidChoice);

        choice.unwrap().count += 1;

        Ok(())
    }

    /// Fonction to delete a proposal
    /// Deletes a proposal if it has ended and has been closed for at least 30 days.
    /// # Arguments
    /// * `ctx` - The context containing the accounts required for deleting the proposal.
    /// # Returns
    /// * `Ok(())` if the proposal is deleted successfully.
    /// * An error if the proposal cannot be deleted due to not being authorized or not meeting the time requirements.
    /// # Errors
    /// * `ProposalError::NotAuthorized` if the signer is not the creator of the proposal.
    /// * `ProposalError::VoteNotEnded` if the proposal has not ended yet.
    /// * `ProposalError::TooRecentToDelete` if the proposal was closed less than 30 days ago.
    ///
    /// # Note
    /// This function checks the current time against the proposal's end date to ensure it has ended.
    /// It also checks if the proposal has been closed for at least 30 days before allowing deletion.
    ///
    pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
        let clock = &ctx.accounts.clock;
        let timestamp = clock.unix_timestamp as u64;
        let proposal = &mut ctx.accounts.proposal;

        require!(proposal.creator == ctx.accounts.signer.key(), ProposalError::NotAuthorized);
        require!(proposal.date_end < timestamp, ProposalError::VoteNotEnded);

        const THIRTY_DAYS: u64 = 2_592_000;
        require!(
            timestamp - proposal.date_end >= THIRTY_DAYS,
            ProposalError::TooRecentToDelete
        );

        proposal.close(ctx.accounts.signer.to_account_info())?;
        msg!("Proposal deleted by: {}", ctx.accounts.signer.key());

        Ok(())
    }
}

/// This module contains the account structures and their associated constraints for the voting program.

/// Context for initializing a proposal
#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitializeProposal<'info> {
    #[account(init, payer = signer, space = 8 + Proposal::INIT_SPACE, seeds = [b"proposal", title.as_bytes()], bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context for casting a vote
#[derive(Accounts)]
pub struct InitializeVote<'info> {
    #[account(init, space = 8 + Voting::INIT_SPACE, payer=signer, seeds = [b"vote", proposal.key().as_ref(), signer.key().as_ref()], bump)]
    pub vote: Account<'info, Voting>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

/// Context for deleting a proposal
#[derive(Accounts)]
pub struct DeleteProposal<'info> {
    #[account(mut, close=signer)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

/// This module contains the account structures and their associated constraints for the voting program.

/// Structures representing the accounts used in the voting program.
#[account]
#[derive(InitSpace)]
pub struct Proposal {
    #[max_len(64)]
    pub description: String,
    #[max_len(64)]
    pub title: String,

    #[max_len(5)]
    pub votes: Vec<Choice>,
    date_start: u64,
    date_end: u64,
    creator: Pubkey,
}

/// Structure representing a choice in a proposal
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Choice {
    #[max_len(64)]
    pub name: String,
    pub count: u16,
}

/// Structure representing a vote cast by a voter
#[account]
#[derive(InitSpace)]
pub struct Voting {
    #[max_len(64)]
    pub choice: String,
    pub voter: Pubkey,
    pub proposal: Pubkey,
}

/// This module contains the error codes used in the voting program.

/// Error codes for the voting program
#[error_code]
pub enum ProposalError {
    #[msg("La date de début est égale ou plus ultérieur à la date de fin.")]
    DateNotConform,

    #[msg("Le nombre de choix doit être compris entre 2 et 5.")]
    InvalidNumberOfChoices,

    #[msg("Ce choix n'existe pas dans cette proposition.")]
    InvalidChoice,

    #[msg("Le sondage n'est pas ouvert.")]
    VoteNotOpen,
    
    #[msg("Le sondage est clôturé.")]
    VoteClosed,

    #[msg("Vous n'êtes pas autorisé à supprimer ce sondage.")]
    NotAuthorized,

    #[msg("Le sondage n'est pas terminé.")]
    VoteNotEnded,

    #[msg("La fermeture du sondage est trop récente pour pouvoir le supprimer.")]
    TooRecentToDelete,
}