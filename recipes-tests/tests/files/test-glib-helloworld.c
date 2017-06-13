#include <errno.h>
#include <string.h>

#include <glib.h>
#include <glib/gstdio.h>
#include <gio/gio.h>

gboolean verbose = FALSE;

static GOptionEntry options[] =
{
  { "verbose", 0, 0, G_OPTION_ARG_NONE, &verbose, "show verbose messages", NULL },
  { NULL, }
};

int main(int argc, char** argv)
{
	GOptionContext* context;
	GError* error = NULL;

	printf("entered main\n");
	fflush(stdout);

	context = g_option_context_new("");
	g_option_context_add_main_entries(context, options, NULL);
	g_option_context_parse(context, &argc, &argv, &error);
	g_option_context_free(context);


	if (error)
	{
		g_fprintf(stderr, "error parsing arguments: %s\n", error->message);
		g_error_free(error);
		return 1;
	}

	g_debug("hello world");
	g_message("hello world");
	g_info("hello world");
	g_critical("hello world");
	g_warning("hello world");
	/*g_error("hello world");*/

	return 0;
}

