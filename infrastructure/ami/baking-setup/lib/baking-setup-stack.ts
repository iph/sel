import * as cdk from '@aws-cdk/core';
import * as ec2 from '@aws-cdk/aws-ec2';

export class BakingSetupStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    let v = new ec2.Vpc(this, "VPC", {
      maxAzs: 3,
      subnetConfiguration: [
        {
          cidrMask: 24,
          name: 'ingress',
          subnetType: ec2.SubnetType.PUBLIC
        }
      ]
    });  
    let sg = new ec2.SecurityGroup(this, "Sg", {
      vpc: v,
    })
    sg.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22), 'allow ssh access from the world');

    this.exportValue(sg.securityGroupId, {
      name: "SecurityGroup"
    });

    this.exportValue(v.vpcId, {
      name: "Vpc"
    });

    let subnets = v.selectSubnets({
      subnetType: ec2.SubnetType.PUBLIC
    });

    
    this.exportValue(subnets.subnetIds[0], {
      name: "SubnetId1"
    });

    this.exportValue(subnets.subnetIds[1], {
      name: "SubnetId2"
    })
  }
}
