# Sel

## Premise

BUY BUY BUY, SELL SELL `SEL`

It's dumb. This is a practice run of using github actions + CI/CD in a code pipeline for a basic server
that runs my `bets` from coinbase (essentially setting limits on when to enter and leave).

Essentially:

- Run integ tests
- Test in a development environment
- Run in production

## Integ tests

One of the integ tests that really intrigues me is the idea of a load test, to ensure quality of what's going out. Let's see if this is possible.

## Test in a development environment

The code should be able to both modify the cfn infra of the development environment, but also:
- create packer ami id
- use packer ami in dev

## Prod 

Same as test