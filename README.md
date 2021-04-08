# Sel

This is mostly an experimental repo to see if I can get a full CI/CD pipeline with cdk and rust, using github actions for the CI and then deploy to a single
devo -> production codepipeline.

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