import { join } from "https://deno.land/std/path/mod.ts";


type EnumConfig = {
    imports: string[];
    variants: {[variant: string]: string[] }
}

interface EnumsConfig {
    [ enumName: string]: EnumConfig
}
type OperationsConfig = { [operation: string]: string };

interface Config {
    imports: string[];
    enums: EnumsConfig;
    // operationImports: string[];
    operations: OperationsConfig;
}

const config: Config = {
    imports: ['use crate::token_type::{Literal};', 'use crate::types::expr::Expr;', 'use crate::types::stmt::Stmt;'],
    enums: {
        Expr: {
            imports: ['use crate::token_type::{Token,Literal};'],
            variants: {
                Assign   : ['name: Token', 'value: Box<Expr>'],
                Binary   : ['left: Box<Expr>', 'operator: Token', 'right: Box<Expr>'],
                Grouping : ['expression: Box<Expr>'],
                Literal  : ['value: Literal'],
                Unary    : ['operator: Token', 'right: Box<Expr>'],
                Variable : ['name: Token']
            }
        },
        Stmt: {
            imports: ['use crate::token_type::{Token,Literal};', 'use crate::types::expr::Expr;'],
            variants: {
                Block      : ['statements: Vec<Stmt>'],
                Expression : ['expression: Box<Expr>'],
                Print      : ['expression: Box<Expr>'],
                Var        : ['name: Token', 'initializer: Box<Expr>'],
            }
        }
    },
    operations: {
        Print: 'String',
        Interpret: 'Literal'
    }
}


function generate(config: Config, outDir: string) {
    for (const enumName of Object.keys(config.enums)) {
        const code = generateEnum(enumName, config.enums[enumName]);
        const filePath = join(outDir, `${enumName.toLowerCase()}.rs`);
        Deno.writeTextFileSync(filePath, code);
    }

    const code = generateOperationImpls(config.operations, config.enums, config.imports, outDir);
    Deno.writeTextFileSync(join(outDir, `operations.rs`), code);
}


function generateEnum(name: string, config: EnumConfig) {
    return [`${config.imports.join('\n')}`,
        ``,
        '#[derive(Debug, PartialEq, Clone, PartialOrd)]',
        `pub enum ${name} {`,
        '    ' +
        `${Object.keys(config.variants).map(varName => `${varName}{${config.variants[varName].join(', ')}},`).join('\n    ')}`,
        `}`].join('\n') + '\n';
}

function generateOperationImpls(operations: OperationsConfig, enums: EnumsConfig, imports: string[], baseDir: string) {
    let code = imports.join('\n') + '\n';
    code =  code + Object.keys(operations).map(operation => `mod ${operation.toLowerCase()};`).join('\n') + '\n';
    for (const operationName of Object.keys(operations)) {
        const type = operations[operationName];
        code = code + generateTraits(operationName, type) + '\n';
    }
    for (const operationName of Object.keys(operations)) {
        const returnType = operations[operationName];

        for (const enumName of Object.keys(enums)) {
            const enumConfig = enums[enumName];

            const branches: string[] = [];
            for (const variant of Object.keys(enumConfig.variants)) {

                generateFnStub(operationName, variant.toLowerCase(), enumConfig.variants[variant], returnType, baseDir, imports);

                const argsNames = enumConfig.variants[variant].map(arg => arg.split(': ')[0]);
                branches.push(
                    `            ${enumName}::${variant} { ${argsNames.join(', ')} } => ${operationName.toLowerCase()}::${variant.toLowerCase()}(${argsNames.join(', ')}),`
                );
            }

            const operationCode =[
                `impl ${operationName} for ${enumName} {`,
                `    fn ${operationName.toLowerCase()}(&self) -> ${returnType} {`,
                `        match self {`,
                `${branches.join('\n')}`,
                `        }`,
                `    }`,
                `}`].join('\n') + '\n';
            // code =
            // + `${code}`
            // + ``
            // + `    ${operationCode}
            // + `;

            code = code + '\n' + operationCode;
        }
    }
    return code;
}

function generateTraits(name: string, type: string): string {
    return [
        `pub trait ${name} {`,
        `    fn ${name.toLowerCase()}(&self) -> ${type};`,
        `}`,
    ].join('\n') + '\n';
}

function generateFnStub(operation: string, variant: string, args: string[], returnType: string, baseDir: string, imports: string[]) {
    const filePath = join(baseDir, 'operations', `${operation}.rs`);
    const borrowedArgs = args.map(arg => {
        const [name, type] = arg.split(': ');
        return `${name}: &${type}`;
    })
    const signature = `pub fn ${variant}(${borrowedArgs.join(', ')}) -> ${returnType}`;

    let code = Deno.readTextFileSync(filePath);
    if (!code.includes(signature)) {
        code = code + '\n' + [
            `${signature} {`,
            '    todo!()',
            '}'
        ].join('\n') + '\n';
    }

    Deno.writeTextFileSync(filePath, code);
}


generate(config, 'D:\\repos\\lox\\rlox\\src\\types');
