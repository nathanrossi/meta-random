#!/usr/bin/env python3

import sys
import gi.repository.GLib

if __name__ == "__main__":
	print("Running a simple mainloop with timeout to quit")

	ml = gi.repository.GLib.MainLoop()
	gi.repository.GLib.timeout_add_seconds(10, ml.quit)

	print("mainloop quit properly")
	sys.exit(0)

