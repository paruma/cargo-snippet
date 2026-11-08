// VS Code 本体の snippetParser.ts で JSON 出力の展開を検証する。
// パーサーの隣で npm install --ignore-scripts typescript@5.9.3 を実行してから:
// node tests/check-vscode.mjs /path/to/snippetParser.ts
// 同じディレクトリに VS Code 本体の charCode.ts も取得しておく。
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createRequire } from 'node:module';

const parserPath = pathToFileURL(process.argv[2]);
const asModule = source => `data:text/javascript;base64,${Buffer.from(source).toString('base64')}`;
const ts = createRequire(parserPath)('typescript');
const transform = source => ts.transpileModule(source, {
    compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ES2022 },
}).outputText;
const charCode = asModule(transform(readFileSync(new URL('./charCode.ts', parserPath), 'utf8')));
const parserSource = readFileSync(parserPath, 'utf8').replace(
    "'../../../../base/common/charCode.js'", JSON.stringify(charCode),
);
const { SnippetParser } = await import(asModule(transform(parserSource)));
const root = fileURLToPath(new URL('../', import.meta.url));
const fixture = `${root}tests/fixtures/vscode_escapes.rs`;
const binary = process.argv[3] ?? `${root}target/debug/cargo-snippet`;
const json = execFileSync(binary, ['snippet', '-t', 'vscode', fixture], { encoding: 'utf8', cwd: root });
const snippets = JSON.parse(json);
assert.deepEqual(Object.keys(snippets), ['escapes']);
const expected = execFileSync('rustfmt', [], {
    input: readFileSync(fixture, 'utf8').replace('#[snippet]\n', ''),
    encoding: 'utf8',
}).replace(/\n$/, '');
const parsed = new SnippetParser().parse(snippets.escapes.body.join('\n'));
assert.equal(parsed.toString(), expected);
assert.equal(parsed.placeholders.length, 0);
let variables = 0;
parsed.walk(marker => {
    if (marker.constructor.name === 'Variable') variables++;
    return true;
});
assert.equal(variables, 0);
for (let count = 0; count <= 8; count++) {
    for (const suffix of ['', '$', '$0', '${1:default}', '$TM_FILENAME', '}', '日本語']) {
        const literal = '\\'.repeat(count) + suffix;
        const body = JSON.parse(JSON.stringify(literal.replaceAll('\\', '\\\\').replaceAll('$', '\\$')));
        assert.equal(new SnippetParser().parse(body).toString(), literal);
    }
}
console.log('VS Code parser: 展開結果が元の整形済みソースと一致しました。');
