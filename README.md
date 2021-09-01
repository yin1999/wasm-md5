# wasm-md5

A WebAssembly md5 digest lib.

## usage

The browser should support ES module.

```html
<!DOCTYPE html>
<html>

<head>
	<meta charset="UTF-8">
</head>

<body>
	<!-- enable ES module -->
	<script type="module"> 
		import init, { Md5Digest } from '//cdn.jsdelivr.net/gh/yin1999/wasm-md5@release/md5_wasm.js'
		init()
			.then(() => {
				const hash = new Md5Digest()
				const enc = new TextEncoder()
				const data = enc.encode("hello world")
				hash.update(data)

				console.log(`md5 digest: ${hash.finalize()}`)
			})
	</script>
</body>

</html>
```
