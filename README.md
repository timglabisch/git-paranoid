[dont_use_teardown_or_setup]
path = 'test'
code = ['tearDown', 'setUp']
ignore_authors = []
ignore_branches = []

[take_care_with_dateTime]
path = '.*'
code = ['new \\DateTime\(\)']
ignore_authors = []
ignore_branches = []

[category]
path = 'Category'
code = ['.*']
ignore_authors = []
ignore_branches = []
