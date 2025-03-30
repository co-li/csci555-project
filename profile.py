"""A profile with two c220g5 nodes connected by a LAN.

Instructions:
Wait for the profile instance to start, and then log in to either host via the
ssh ports specified below.
"""

import geni.portal as portal
import geni.rspec.pg as rspec

request = portal.context.makeRequestRSpec()

# Create two raw "PC" nodes
node1 = request.RawPC("node1")
node2 = request.RawPC("node2")

# Set each of the two to specifically request "c220g5" nodes (CloudLab Wisc)
node1.hardware_type = "c220g5"
node2.hardware_type = "c220g5"

# Set both nodes to use a Ubuntu 24.04 image 
node1.disk_image = "urn:publicid:IDN+emulab.net+image+emulab-ops//UBUNTU24-64-STD";
node2.disk_image = "urn:publicid:IDN+emulab.net+image+emulab-ops//UBUNTU24-64-STD";

# Set up node 1 with Rust
node1.addService(rspec.Execute(shell="bash", command="/local/repository/setup_scripts/rust_setup.sh"))

# Set up node 2 with k6
node2.addService(rspec.Execute(shell="bash", command="/local/repository/setup_scripts/k6_setup.sh"))

# Create a link between them
link1 = request.Link(members = [node1, node2])

portal.context.printRequestRSpec()
