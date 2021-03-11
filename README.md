# Warpsite

## Architecture

Working off of [this](https://gist.github.com/warpfork/79edc135e049f1c43142a7e14c8beb31) as a design doc.

### File Structure

Warpsite works off a root project folder, containing the content of the site.
Any generator-specific files are in dotfiles or dotdirs.

Site file hierarchy (the `PageTree`) follows directory hierarchy.
**TODO**: add some sort of meta file to change display name of directory.

Markdown files (*.md) are used as main site content (called `Pages`).
**TODO**: should display name control be in-band (in the markdown, like a page directive), or out-of-band (a dotfile?)

All other files (called `Resources`) will be included (in relative place) in the output tree.

Ignoring files is done with `.warpignore` files, which follow gitignore rules.
Subdirs can have their own `.warpignore` files which will apply to the directory tree underneath them.

### Processing and Rendering

The above section only deals with determining What Is Content and perhaps What Do I Name Content, it has so far
not inspected the contents of any of the files, or attempted to render anything.

Useful things to process:
* Headers (and anchor creation) in docs, to more fully flesh out the Site Tree and aid in TOC creation later.
* In-band metadata (author, date, title, I dunno) to hand to the templating
* The page "content" as one blob, I'd rather not try to get into the business of auto-chunking blocks of
  text and pretending I know better than the user
* Stretch goal: tree shake any resources that don't actually get referenced
* Probably not worth optimizing: Hash Resources and only redeploy to output if they aren't there,
  add hash to filename and rewrite any refs

Templating:
* Probably [Tera](https://github.com/Keats/tera)? Maybe liquid?
* Whether or not the templating sucks to work with seems to come down to the building blocks available in the templates
* Default hard-coded template that just chucks the markdown render into appropriate tags, unstyled and ugly, but alive
* User-controllable chunks: Nav, TOC (preferably scopable and with controlled depth?), Page Content?
* Themes are dirs under `.warpsite/themes/`, default can be set or can be controlled per page? per directory? both?
* Themes can be extended either via `block`s or just straight overriding a whole template, but these should live outside
  the theme dir, maybe closer to their use site?
