"""A profile with two c6525-25g nodes connected by a LAN.

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

# Set each of the two to specifically request "c6525-25g" nodes (CloudLab Utah)
node1.hardware_type = "c6525-25g"
node2.hardware_type = "c6525-25g"

# Set both nodes to use a Ubuntu 24.04 image 
node1.disk_image = "urn:publicid:IDN+emulab.net+image+emulab-ops//UBUNTU24-64-STD";
node2.disk_image = "urn:publicid:IDN+emulab.net+image+emulab-ops//UBUNTU24-64-STD";

# Set up node 1 with Rust
node1.addService(rspec.Execute(shell="bash", command="sudo apt-get update; sudo apt-get install -y rustc cargo"))

# Set up node 2 with k6
node2.addService(rspec.Execute(shell="bash", command="sudo gpg -k; sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69; echo \"deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main\" | sudo tee /etc/apt/sources.list.d/k6.list; sudo apt-get update; sudo apt-get install -y k6"))

# Create a link between them
link1 = request.Link(members = [node1, node2])

portal.context.printRequestRSpec()
