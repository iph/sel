import { expect as expectCDK, matchTemplate, MatchStyle } from '@aws-cdk/assert';
import * as cdk from '@aws-cdk/core';
import * as BakingSetup from '../lib/baking-setup-stack';

test('Empty Stack', () => {
    const app = new cdk.App();
    // WHEN
    const stack = new BakingSetup.BakingSetupStack(app, 'MyTestStack');
    // THEN
    expectCDK(stack).to(matchTemplate({
      "Resources": {}
    }, MatchStyle.EXACT))
});
