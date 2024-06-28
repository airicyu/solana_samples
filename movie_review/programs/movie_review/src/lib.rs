use anchor_lang::prelude::*;

declare_id!("2JnY2Fgxyv1bNq2dugGiMQTib3Wyxs8bszcX51tw3BtH");

#[program]
pub mod movie_review {
    use super::*;

    pub fn create_review(
        ctx: Context<CreateReviewContext>,
        data: MoviewReviewIxInput
    ) -> Result<()> {
        let review_record = &mut ctx.accounts.review_record;
        review_record.title = data.title.clone();
        review_record.review = data.review.clone();

        msg!(
            "Review created successfully. Title: {}, Review: {}",
            review_record.title,
            review_record.review
        );
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(data: MoviewReviewIxInput)]
pub struct CreateReviewContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [b"movieReview:".as_ref(), data.title.as_ref()],
        bump,
        space = 8 + ReviewRecord::INIT_SPACE
    )]
    pub review_record: Account<'info, ReviewRecord>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct ReviewRecord {
    #[max_len(30)]
    pub title: String,
    #[max_len(50)]
    pub review: String,
}

#[account]
pub struct MoviewReviewIxInput {
    pub title: String,
    pub review: String,
}
