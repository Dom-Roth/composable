use composable_support::math::safe::{
	safe_multiply_by_rational, SafeAdd, SafeDiv, SafeMul, SafeSub,
};
use frame_support::ensure;
use rust_decimal::{
	prelude::{FromPrimitive, ToPrimitive},
	Decimal, MathematicalOps, RoundingStrategy,
};
use sp_runtime::{
	traits::{IntegerSquareRoot, One, Zero},
	ArithmeticError, DispatchError, PerThing,
};

/// From https://balancer.fi/whitepaper.pdf, equation (2)
/// Compute the spot price of an asset pair.
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `base_unit` the unit normalized to the base asset decimal
pub fn compute_spot_price<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	base_unit: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let wi: u32 = wi.deconstruct().into();
	let wo: u32 = wo.deconstruct().into();
	let weight_sum = wi.safe_add(&wo)?;
	let expected_weight_sum: u32 = T::one().deconstruct().into();

	// TODO (vim): This validation must be done at the pallet level then here as there could be more
	//  assets  and weights in the pool than what's here.
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let base_unit = Decimal::from_u128(base_unit).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let full_perthing =
		Decimal::from_u32(T::one().deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let wi_numerator = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let wi = wi_numerator.safe_div(&full_perthing)?;
	let wo_numerator = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let wo = wo_numerator.safe_div(&full_perthing)?;
	let bi_div_wi = bi.safe_div(&wi)?;
	let bo_div_wo = bo.safe_div(&wo)?;
	let spot_price = bi_div_wi.safe_div(&bo_div_wo)?;
	spot_price.safe_mul(&base_unit)?.to_u128().ok_or(ArithmeticError::Overflow)
}

/// From https://balancer.fi/whitepaper.pdf, equation (15)
/// Compute the amount of base asset (out) given the quote asset (in).
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `ai` the quote amount to trade
pub fn compute_out_given_in<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	ai: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let wi: u32 = wi.deconstruct().into();
	let wo: u32 = wo.deconstruct().into();
	let weight_sum = wi.safe_add(&wo)?;
	let expected_weight_sum: u32 = T::one().deconstruct().into();
	// TODO (vim): This validation must be done at the pallet level then here as there could be more
	//  assets  and weights in the pool than what's here.
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let ai = Decimal::from_u128(ai).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let weight_numerator = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let weight_denominator = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let weight_power = weight_numerator.safe_div(&weight_denominator)?;
	let bi_div_bi_plus_ai = bi.safe_div(&bi.safe_add(&ai)?)?;
	let term_to_weight_power =
		bi_div_bi_plus_ai.checked_powd(weight_power).ok_or(ArithmeticError::Overflow)?;
	let one_minus_term = Decimal::one().safe_sub(&term_to_weight_power)?;
	let ao = bo.safe_mul(&one_minus_term)?.to_u128().ok_or(ArithmeticError::Overflow)?;
	Ok(ao)
}

/// From https://balancer.fi/whitepaper.pdf, equation (20)
/// Compute the amount of quote asset (in) given the expected amount of base asset (out).
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `ai` the quote amount to trade
pub fn compute_in_given_out<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	ao: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let wi: u32 = wi.deconstruct().into();
	let wo: u32 = wo.deconstruct().into();
	let weight_sum = wi.safe_add(&wo)?;
	let expected_weight_sum: u32 = T::one().deconstruct().into();

	// TODO (vim): This validation must be done at the pallet level then here as there could be more
	//  assets  and weights in the pool than what's here.
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let ao = Decimal::from_u128(ao).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let weight_numer = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let weight_denom = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let weight_power = weight_numer.safe_div(&weight_denom)?;
	let bo_div_bo_minus_ao = bo.safe_div(&bo.safe_sub(&ao)?)?;
	let term_to_weight_power =
		bo_div_bo_minus_ao.checked_powd(weight_power).ok_or(ArithmeticError::Overflow)?;
	let term_minus_one = term_to_weight_power.safe_sub(&Decimal::one())?;
	let ai = bi.safe_mul(&term_minus_one)?.to_u128().ok_or(ArithmeticError::Overflow)?;
	Ok(ai)
}

/// Compute the amount of the input token given the amount of the output token.
///
/// If `Ok`, returns a tuple containing `(a_sent, fee)`.
/// To get `a_sent` without accounting for the fee, set `f = 0`.
/// Amount in, round up results.
///
/// **NOTE:** Weights must already be normalized.
///
/// # Parameters
/// * `w_i` - Weight of the input token
/// * `w_o` - Weight of the output token
/// * `b_i` - Balance of the input token
/// * `b_o` - Balance of the output token
/// * `a_out` - Amount of the output token desired by the user
/// * `f` - Total swap fee
///
/// From https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#41-fee-math-updates,
/// equation (3)
pub fn compute_in_given_out_new<T: PerThing>(
	w_i: T,
	w_o: T,
	b_i: u128,
	b_o: u128,
	a_out: u128,
	f: T,
) -> Result<(u128, u128), InGivenOutError> {
	ensure!(a_out <= b_o, InGivenOutError::CanNotTakeMoreThanAvailable);
	let w_i = Decimal::from(w_i.deconstruct().into());
	let w_o = Decimal::from(w_o.deconstruct().into());
	let b_i = Decimal::from(b_i);
	let b_o = Decimal::from(b_o);
	let a_out = Decimal::from(a_out);

	let weight_ratio = w_o.safe_div(&w_i)?;
	// NOTE(connor): Use if to prevent pointless conversions if `f` is zero
	let left_from_fee = if f.is_zero() {
		Decimal::ONE
	} else {
		Decimal::from(f.left_from_one().deconstruct().into())
			.safe_div(&Decimal::from(T::one().deconstruct().into()))?
	};
	let b_i_over_fee = b_i.safe_div(&left_from_fee)?;
	let fee = Decimal::ONE.safe_sub(&left_from_fee)?;

	let value = b_o.safe_sub(&a_out)?;
	let value = b_o.safe_div(&value)?;
	let value = value.checked_powd(weight_ratio).ok_or(ArithmeticError::Overflow)?;
	let value = value.safe_sub(&Decimal::ONE)?;

	let a_sent = round_up(b_i_over_fee.safe_mul(&value)?);
	let fee = round_up(a_sent.safe_mul(&fee)?).to_u128().ok_or(ArithmeticError::Overflow)?;

	Ok((a_sent.to_u128().ok_or(ArithmeticError::Overflow)?, fee))
}

#[derive(Debug, Eq, PartialEq)]
pub enum InGivenOutError {
	ArithmeticError(ArithmeticError),
	CanNotTakeMoreThanAvailable,
}

impl From<ArithmeticError> for InGivenOutError {
	fn from(error: ArithmeticError) -> Self {
		InGivenOutError::ArithmeticError(error)
	}
}

impl From<InGivenOutError> for DispatchError {
	fn from(error: InGivenOutError) -> Self {
		match error {
			InGivenOutError::ArithmeticError(error) => DispatchError::from(error),
			InGivenOutError::CanNotTakeMoreThanAvailable => 
				DispatchError::from(
					"`a_out` must not be greater than `b_o` (can't take out more than what's available)"
				),
		}
	}
}

/// Rounds a decimal value up to the nearest whole number
fn round_up(decimal: Decimal) -> Decimal {
	round(decimal, RoundingStrategy::AwayFromZero)
}

/// Rounds a decimal value to a whole number based on the provided `RoundingStrategy`
fn round(decimal: Decimal, rounding_strategy: RoundingStrategy) -> Decimal {
	decimal.round_dp_with_strategy(0, rounding_strategy)
}

/// https://uniswap.org/whitepaper.pdf, equation (13)
/// Compute the initial share of an LP provider.
/// - `base_amount` the base asset amount deposited.
/// - `quote_amount` the quote asset amount deposited.
#[inline(always)]
pub fn compute_first_deposit_lp(
	base_amount: u128,
	quote_amount: u128,
) -> Result<u128, ArithmeticError> {
	/* TODO (vim): Possible attack vector exists: From https://uniswap.org/whitepaper.pdf
		The formula ensures that a liquidity pool share will never be worth less than
	the geometric mean of the reserves in that pool. However, it is possible for the value of
	a liquidity pool share to grow over time, either by accumulating trading fees or through
	“donations” to the liquidity pool. In theory, this could result in a situation where the value
	of the minimum quantity of liquidity pool shares (1e-18 pool shares) is worth so much that
	it becomes infeasible for small liquidity providers to provide any liquidity
	To mitigate this, Uniswap v2 burns the first 1e-15 (0.000000000000001) pool shares that
		are minted (1000 times the minimum quantity of pool shares), sending them to the zero
		address instead of to the minter. This should be a negligible cost for almost any token
		pair.11 But it dramatically increases the cost of the above attack. In order to raise the
		value of a liquidity pool share to $100, the attacker would need to donate $100,000 to the
		pool, which would be permanently locked up as liquidity.
			 */
	base_amount
		.integer_sqrt_checked()
		.ok_or(ArithmeticError::Overflow)?
		.safe_mul(&quote_amount.integer_sqrt_checked().ok_or(ArithmeticError::Overflow)?)
}

/// https://uniswap.org/whitepaper.pdf, equation (12)
/// Compute the share of an LP provider for an existing, non-empty pool.
/// - `lp_total_issuance` the total LP already issued to other LP providers.
/// - `base_amount` the base amount provided by the current LP provider.
/// - `pool_base_aum` the pool base asset under management.
/// - `pool_quote_aum` the pool quote asset under management.
#[inline(always)]
pub fn compute_deposit_lp(
	lp_total_issuance: u128,
	base_amount: u128,
	quote_amount: u128,
	pool_base_aum: u128,
	pool_quote_aum: u128,
) -> Result<(u128, u128), ArithmeticError> {
	let first_deposit = lp_total_issuance.is_zero();
	if first_deposit {
		let lp_to_mint = compute_first_deposit_lp(base_amount, quote_amount)?;
		Ok((quote_amount, lp_to_mint))
	} else {
		let overwritten_quote_amount =
			safe_multiply_by_rational(pool_quote_aum, base_amount, pool_base_aum)?;
		let lp_to_mint = safe_multiply_by_rational(lp_total_issuance, base_amount, pool_base_aum)?;
		Ok((overwritten_quote_amount, lp_to_mint))
	}
}
