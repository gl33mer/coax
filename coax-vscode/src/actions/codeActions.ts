/**
 * Code Actions Provider
 * 
 * Provides quick-fix actions for Coax findings.
 */

import * as vscode from 'vscode';
import { diagnosticCollection } from '../diagnostics';

/**
 * Coax code action provider implementation
 */
class CoaxCodeActionProvider implements vscode.CodeActionProvider {
    public static readonly providedKinds = [
        vscode.CodeActionKind.QuickFix,
        vscode.CodeActionKind.Refactor,
    ];

    public provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];

        for (const diagnostic of context.diagnostics) {
            if (diagnostic.source !== 'Coax') {
                continue;
            }

            const code = diagnostic.code as string || 'UNKNOWN';

            // Remove finding action
            const removeAction = this.createRemoveAction(document, diagnostic);
            actions.push(removeAction);

            // Add to allowlist action
            const allowlistAction = this.createAllowlistAction(document, diagnostic, code);
            actions.push(allowlistAction);

            // Type-specific actions
            if (code.includes('AWS')) {
                const envAction = this.createEnvVarAction(document, diagnostic, 'AWS_ACCESS_KEY_ID');
                actions.push(envAction);
            }

            if (code.includes('GITHUB')) {
                const envAction = this.createEnvVarAction(document, diagnostic, 'GITHUB_TOKEN');
                actions.push(envAction);
            }

            if (code.includes('STRIPE')) {
                const envAction = this.createEnvVarAction(document, diagnostic, 'STRIPE_SECRET_KEY');
                actions.push(envAction);
            }

            // Unicode-specific actions
            if (code.includes('UNICODE')) {
                const replaceAction = this.createReplaceAsciiAction(document, diagnostic);
                actions.push(replaceAction);
            }

            // Ignore for session action
            const ignoreAction = this.createIgnoreAction(document, diagnostic);
            actions.push(ignoreAction);
        }

        return actions;
    }

    private createRemoveAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Remove this finding',
            vscode.CodeActionKind.QuickFix
        );

        const edit = new vscode.WorkspaceEdit();
        edit.delete(document.uri, diagnostic.range);
        action.edit = edit;
        action.diagnostics = [diagnostic];
        action.isPreferred = true;

        return action;
    }

    private createEnvVarAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic,
        envVarName: string
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            `Replace with \${${envVarName}}`,
            vscode.CodeActionKind.Refactor
        );

        const edit = new vscode.WorkspaceEdit();
        edit.replace(document.uri, diagnostic.range, `\${${envVarName}}`);
        action.edit = edit;
        action.diagnostics = [diagnostic];

        return action;
    }

    private createReplaceAsciiAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Replace with ASCII equivalent',
            vscode.CodeActionKind.QuickFix
        );

        // For now, just remove the problematic character
        // A more sophisticated implementation would analyze and suggest replacements
        const edit = new vscode.WorkspaceEdit();
        edit.delete(document.uri, diagnostic.range);
        action.edit = edit;
        action.diagnostics = [diagnostic];

        return action;
    }

    private createIgnoreAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Ignore for this session',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'coax.ignoreFinding',
            title: 'Ignore Finding',
            arguments: [document.uri, diagnostic.range, diagnostic.code],
        };

        return action;
    }

    private createAllowlistAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic,
        code: string
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Add to allowlist (.coax.yaml)',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'coax.addToAllowlist',
            title: 'Add to Allowlist',
            arguments: [document.uri, diagnostic.range, code],
        };

        return action;
    }
}

/**
 * Initialize the code action provider
 */
export function initializeCodeActionProvider(context: vscode.ExtensionContext): void {
    const selector = { language: '*', scheme: '*' };
    const provider = new CoaxCodeActionProvider();

    context.subscriptions.push(
        vscode.languages.registerCodeActionsProvider(selector, provider, {
            providedCodeActionKinds: CoaxCodeActionProvider.providedKinds,
        })
    );
}
