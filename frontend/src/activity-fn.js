export function prefix(location, ...prefixes) {
  return prefixes.some(
    prefix => (
      location.href.indexOf(`${location.origin}/${prefix}`) !== -1
    )
  )
}

/**
 * Always return true for now
 * @param location
 * @returns {boolean}
 */
export function apm(location) {
  return true;
}
