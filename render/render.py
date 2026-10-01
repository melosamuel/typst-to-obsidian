from classes import JsonNode
from render_math import render_math


def render_heading(node: JsonNode) -> str:
  marker = ''
  content = ''

  for child in node.get('children', []):
    if child['kind'] == 'HeadingMarker':
      marker = child.get('text', '')

    elif child['kind'] == 'Markup':
      content = render(child)

  level = len(marker)
  return f'{"#" * level} {content}'


def render_strong(node: JsonNode) -> str:
  content = ''
  for child in node.get('children', []):
    if child['kind'] == 'Star':
      continue
    elif child['kind'] == 'Markup':
      content = render(child)
  return f'**{content}**'


def render_emphasis(node: JsonNode) -> str:
  content = ''
  for child in node.get('children', []):
    if child['kind'] == 'Underscore':
      continue
    elif child['kind'] == 'Markup':
      content = render(child)
  return f'*{content}*'


def render_function(node: JsonNode) -> str:
  content = ''
  fname = ''
  selected = ''
  options = {
    'highlight': ['==', '=='],
    'lorem': ['', ''],
    'lower': ['', ''],
    'overline': ['$\\overline{\\text{', '}}$'],
    'smallcaps': ['<span style="font-variant:small-caps;">', '</span>'],
    'strike': ['~~', '~~'],
    'sub': ['$_{\\text{', '}}$'],
    'super': ['$^{\\text{', '}}$'],
    'underline': ['<u>', '</u>'],
    'upper': ['', ''],
  }
  for child in node.get('children', []):
    if child['kind'] == 'Ident':
      fname = child.get('text', '')
      selected = options[fname]
    elif child['kind'] == 'Args':
      content = render(child)
  if fname == 'lorem':
    return node.get('text', '')
  if fname == 'lower':
    content = content.lower()
  if fname == 'upper':
    content = content.upper()

  return selected[0] + content[1:-1].strip('"') + selected[1]


def render(node: JsonNode) -> str:
  if node['kind'] == 'Equation':
    return render_math(node)

  if node['kind'] == 'Heading':
    return render_math(node)

  if node['kind'] == 'Heading':
    return render_heading(node)

  if node['kind'] == 'Strong':
    return render_strong(node)

  if node['kind'] == 'Emph':
    return render_emphasis(node)

  if node['kind'] == 'Hash':
    return ''

  if node['kind'] == 'FuncCall':
    return render_function(node)

  if node['kind'] == 'Linebreak':
    return '<br/>'

  if 'children' in node:
    return ''.join(render(child) for child in node['children'])

  return node.get('text', '')
