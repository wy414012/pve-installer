package Proxmox::Sys;

use strict;
use warnings;

# The HTML specification actually gives a "blessed" regex for email addresses:
# https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
# Using that /should/ cover all possible cases that are encountered in the wild.
our $EMAIL_RE = '^[a-zA-Z0-9.!#$%&\'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$';