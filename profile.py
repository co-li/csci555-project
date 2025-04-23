"""A profile with two Ubuntu nodes connected by a LAN.

Instructions:
Wait for the profile instance to start, and then log in to either host via the
ssh ports specified below.
"""

import geni.portal as portal
import geni.rspec.pg as rspec

# Crete a portal context, needed to define parameters
pc = portal.Context()

# Create a Request object to start building the RSpec.
request = pc.makeRequestRSpec()

# Optional physical type for all nodes.
pc.defineParameter("phystype",  "Optional physical node type",
                   portal.ParameterType.NODETYPE, "",
                   longDescription="Pick a single physical node type (pc3000,d710,etc) " +
                   "instead of letting the resource mapper choose for you.")

# Optional link speed, normally the resource mapper will choose for you based on node availability
pc.defineParameter("linkSpeed", "Link Speed", portal.ParameterType.INTEGER, 0,
                   [(0,"Any"),(100000,"100Mb/s"),(1000000,"1Gb/s"),(10000000,"10Gb/s"),(25000000,"25Gb/s"),(100000000,"100Gb/s")],
                   advanced=True,
                   longDescription="A specific link speed to use for your lan. Normally the resource " +
                   "mapper will choose for you based on node availability and the optional physical type.")

# Retrieve the values the user specifies during instantiation.
params = pc.bindParameters()

# Create two raw "PC" nodes
node1 = request.RawPC("node1")
node2 = request.RawPC("node2")

# Set each node to the requested node type
if params.phystype != "":
    node1.hardware_type = params.phystype
    node2.hardware_type = params.phystype

# Set both nodes to use a Ubuntu 24.04 image 
node1.disk_image = "urn:publicid:IDN+emulab.net+image+emulab-ops//UBUNTU24-64-STD";
node2.disk_image = "urn:publicid:IDN+emulab.net+image+emulab-ops//UBUNTU24-64-STD";

# Set up node 1 with Rust
node1.addService(rspec.Execute(shell="bash", command="/local/repository/setup_scripts/rust_setup.sh"))

# Set up node 2 with k6
node2.addService(rspec.Execute(shell="bash", command="/local/repository/setup_scripts/k6_setup.sh"))

# Create a link between them
lan = request.Link(members = [node1, node2])

if params.linkSpeed > 0:
    lan.bandwidth = params.linkSpeed

pc.printRequestRSpec(request)
