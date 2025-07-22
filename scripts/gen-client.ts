import {createFromRoot} from 'codama';
import {rootNodeFromAnchor} from '@codama/nodes-from-anchor';
import {renderRustVisitor} from '@codama/renderers';
import {command, run, string, number, positional, option} from 'cmd-ts';
import {readFileSync} from "fs";

// import orcaIdl from "../idl/idl.json";

const cmd = command({
    name: 'gen-idl',
    description: 'Generate IDL',
    version: '1.0.0',
    args: {
        idl_path: positional({type: string, displayName: "idl-path"}),
        result_path: positional({type: string, displayName: "result-path"}),
    },
    handler: (args) => {
        args.idl_path; // string
        args.result_path; // number
        createClient(args.idl_path, args.result_path)
    },
});

function createClient(idl_path: string, result_path: string) {
    const json = JSON.parse(readFileSync(idl_path).toString());
    // @ts-ignore
    const codama = createFromRoot(rootNodeFromAnchor(json));
    codama.accept(renderRustVisitor(result_path));
}

run(cmd, process.argv.slice(2)).then(() => {
    console.log("Done");
    process.exit(0);
}).catch(e => {
    console.error("Error:", e);
    process.exit(1);
})


// createClient(orcaIdl, "orca");
