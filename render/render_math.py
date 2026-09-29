from classes import JsonNode


def render_equation(node: JsonNode) -> str:
  content = ''

  for child in node.get('children', []):
    if child['kind'] == 'Dollar':
      continue
    content += render_math(child)

  return f'${content}$'


def render_math_call(node: JsonNode) -> str | None:
  content = ''
  fname = ''
  options = {
    'grave': ['\\grave{', '}'],
  }

  for child in node.get('children', []):
    if child['kind'] == 'MathIdent':
      fname = child.get('text', '')

    elif child['kind'] == 'MathArgs':
      for argument in child.get('children', []):
        if argument['kind'] in ['LeftParen', 'RightParen']:
          continue
        content += render_math(argument)

  if fname not in options:
    return None

  selected = options[fname]
  return selected[0] + content + selected[1]


def render_math_ident(node: JsonNode) -> str:
  name = node.get('text', '')
  options = {
    'rho': '\\rho',
  }

  if name in options:
    return options[name]

  return name


def render_math(node: JsonNode) -> str:
  if node['kind'] == 'Equation':
    return render_equation(node)

  if node['kind'] == 'MathCall':
    content = render_math_call(node)
    if content is not None:
      return content

  if node['kind'] == 'MathIdent':
    return render_math_ident(node)

  if 'children' in node:
    return ''.join(render_math(child) for child in node['children'])

  return node.get('text', '')
