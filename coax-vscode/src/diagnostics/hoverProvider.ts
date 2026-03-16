/**
 * Hover Provider
 * 
 * Shows detailed finding information on hover.
 */

import * as vscode from 'vscode';
import { diagnosticCollection } from './index';

/**
 * Coax hover provider implementation
 */
class CoaxHoverProvider implements vscode.HoverProvider {
    public provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.Hover | undefined {
        const diagnostics = diagnosticCollection.get(document.uri);
        if (!diagnostics || diagnostics.length === 0) {
            return undefined;
        }

        for (const diagnostic of diagnostics) {
            if (diagnostic.range.contains(position)) {
                const contents = new vscode.MarkdownString();
                contents.isTrusted = true;
                
                const code = diagnostic.code as string || 'UNKNOWN';
                const severityName = this.getSeverityName(diagnostic.severity);
                
                contents.appendMarkdown(`#### 🔒 ${code}\n\n`);
                contents.appendMarkdown(`**Severity:** ${severityName}\n\n`);
                contents.appendMarkdown(`${diagnostic.message}\n\n`);
                contents.appendMarkdown(`**Recommendation:** ${this.getRecommendation(code)}\n\n`);
                contents.appendMarkdown(`[View Documentation](https://github.com/gl33mer/coax/blob/main/docs/HANDOFF.md)`);

                return new vscode.Hover(contents);
            }
        }

        return undefined;
    }

    private getSeverityName(severity: vscode.DiagnosticSeverity): string {
        switch (severity) {
            case vscode.DiagnosticSeverity.Error:
                return 'High';
            case vscode.DiagnosticSeverity.Warning:
                return 'Medium';
            case vscode.DiagnosticSeverity.Information:
                return 'Low';
            default:
                return 'Unknown';
        }
    }

    private getRecommendation(code: string): string {
        const recommendations: Record<string, string> = {
            'AWS_ACCESS_KEY': 'Remove immediately and rotate via AWS IAM Console. Use IAM roles or environment variables instead.',
            'AWS_SECRET_KEY': 'Remove immediately and rotate via AWS IAM Console. Never commit secret keys.',
            'GITHUB_PAT': 'Revoke this token immediately via GitHub Settings. Use GitHub Actions secrets instead.',
            'GITHUB_TOKEN': 'Remove from code. Use GitHub Actions secrets or environment variables.',
            'STRIPE_KEY': 'Rotate this key in Stripe Dashboard. Use environment variables for API keys.',
            'PRIVATE_KEY': 'Remove immediately. This appears to be a private key that should never be committed.',
            'GENERIC_SECRET': 'Remove this secret and use environment variables or a secrets manager.',
            'UNICODE-HOMOGLYPH': 'Replace with ASCII equivalent. This appears to be a mixed-script identifier.',
            'UNICODE-INVISIBLE': 'Remove this invisible character. It may be used to hide malicious content.',
            'UNICODE-BIDI': 'Remove this bidirectional override character. It may be used to hide content.',
        };
        return recommendations[code] || 'Remove this finding and review the code for security issues.';
    }
}

/**
 * Initialize the hover provider
 */
export function initializeHoverProvider(context: vscode.ExtensionContext): void {
    const selector = { language: '*', scheme: '*' };
    
    context.subscriptions.push(
        vscode.languages.registerHoverProvider(selector, new CoaxHoverProvider())
    );
}
