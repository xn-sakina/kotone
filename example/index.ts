import { transform } from 'kotone'
import 'zx/globals'

const run = async () => {
  const file = path.join(__dirname, './test.csv')
  const originContent = fs.readFileSync(file, 'utf-8')
  console.log(originContent);
  const buffer = fs.readFileSync(file)
  const result = await transform(buffer)
  console.log(result)
}

run()
