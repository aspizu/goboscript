'use strict';

import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    vscode.languages.registerDocumentFormattingEditProvider('foo-lang', {
        provideDocumentFormattingEdits(document: vscode.TextDocument): vscode.TextEdit[] {
            const firstLine = document.lineAt(0);
            if (firstLine.text !== '42') {
                return [vscode.TextEdit.insert(firstLine.range.start, '42\n')];
            }
            return [];
        }
    });
}
