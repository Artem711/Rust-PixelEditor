async function main() {
  const lib = await import('../pkg/index.js').catch(console.error)
  const image = new lib.Image()
  console.log(image)
}

main()
