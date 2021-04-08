variable "aws_access_key" {
  type    = string
  default = ""
}

variable "aws_secret_key" {
  type    = string
  default = ""
}

variable "subnet" {
  type = string
}

variable "sg" {
  type = string
}

variable "default_ami" {
  type = string
  default = "ami-0d5d1a3aa3516231f"
}

# "timestamp" template function replacement
locals { timestamp = regex_replace(timestamp(), "[- TZ:]", "") }

# source blocks are generated from your builders; a source can be referenced in
# build blocks. A build block runs provisioner and post-processors on a
# source. Read the documentation for source blocks here:
# https://www.packer.io/docs/from-1.5/blocks/source
source "amazon-ebs" "core" {
  access_key    = "${var.aws_access_key}"
  ami_name      = "packer-example ${local.timestamp}"
  instance_type = "t3.micro"
  region        = "us-west-2"
  secret_key    = "${var.aws_secret_key}"
  source_ami    = "${var.default_ami}"
  ssh_username  = "ec2-user"

  security_group_id = "${var.sg}"
  subnet_id = "${var.subnet}"
  associate_public_ip_address = true
  
}

# a build block invokes sources and runs provisioning steps on them. The
# documentation for build blocks can be found here:
# https://www.packer.io/docs/from-1.5/blocks/build
build {
  sources = ["source.amazon-ebs.core"]

  provisioner "file" {
    source = "srv/target/release/srv"
    destination = "/tmp/srv"
  }

  provisioner "file" {
    source = "core.service"
    destination = "/tmp/core.service"
  }

  provisioner "shell" {
    inline = [
      "sudo cp /tmp/srv /usr/bin/",
      "sudo cp /tmp/core.service /lib/systemd/system/core.service",
      "sudo cp /tmp/core.service /etc/systemd/system/core.service",
      "sudo chmod 644 /etc/systemd/system/core.service",
      "sudo systemctl enable core"
    ]
  }

}

