# 📋 Anchor Security Cheat Sheet

A quick reference for the most common security constraints in the Anchor framework.

## Account Types

| Type | When to use | Security Property |
|:---|:---|:---|
| `Account<'info, T>` | Standard data accounts | Automatically checks that `account.owner == program_id`. |
| `Signer<'info>` | Required for authorization | Automatically checks that `account.is_signer == true`. |
| `Program<'info, T>` | Calling other programs | Automatically checks the account is the correct Program ID. |
| `UncheckedAccount<'info>` | Extremely rare cases | **NO CHECKS.** Use with extreme caution and manual validation. |

## Common Constraints

| Constraint | Purpose | Example |
|:---|:---|:---|
| `mut` | Allows modifying data | `#[account(mut)]` |
| `has_one` | Links two accounts | `#[account(has_one = authority)]` (Checks `data.authority == auth_account.key`) |
| `seeds` / `bump` | Validates a PDA | `#[account(seeds = [b"user"], bump)]` |
| `constraint` | Custom logic check | `#[account(constraint = user.age >= 18)]` |
| `owner` | Manual owner check | `#[account(owner = token_program.key())]` |
| `address` | Checks specific key | `#[account(address = crate::ID)]` |

## Arithmetic Reference

| Dangerous | Secure | Why? |
|:---|:---|:---|
| `a + b` | `a.checked_add(b)?` | Returns an Error on overflow instead of wrapping. |
| `a - b` | `a.checked_sub(b)?` | Returns an Error on underflow. |
| `a * b` | `a.checked_mul(b)?` | Returns an Error on overflow. |
| `a / b` | `a.checked_div(b)?` | Returns an Error on division by zero. |

## Professional Tip: Custom Errors
Always use `ok_or(error!(ErrorCode::MyError))` with checked arithmetic for clear logs.
