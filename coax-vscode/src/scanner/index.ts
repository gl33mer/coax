/**
 * Scanner Service
 * 
 * Handles Coax CLI binary execution and result parsing.
 */

import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { execFile } from 'child_process';
import { minimatch } from 'minimatch';

export interface Finding {
    file: string;
    line: number;
    column: number;
    endLine: number;
    endColumn: number;
    type: string;
    severity: 'critical' | 'high' | 'medium' | 'low';
    message: string;
    recommendation: string;
    code: string;
}

export interface ScanResult {
    findings: Finding[];
    metadata: {
        filesScanned: number;
        duration: number;
        timestamp: string;
    };
}

let context: vscode.ExtensionContext | null = null;

/**
 * Initialize the scanner with extension context
 */
export function initializeScanner(ctx: vscode.ExtensionContext): void {
    context = ctx;
}

/**
 * Dispose scanner resources
 */
export function disposeScanner(): void {
    context = null;
}

/**
 * Get the path to the Coax binary
 */
function getBinaryPath(): string {
    const config = vscode.workspace.getConfiguration('coax');
    const customPath = config.get<string>('binaryPath', '');

    if (customPath && fs.existsSync(customPath)) {
        return customPath;
    }

    if (!context) {
        throw new Error('Extension context not initialized');
    }

    const plat = process.platform;
    const architecture = process.arch;

    let platformFolder: string;
    switch (plat) {
        case 'darwin':
            platformFolder = `darwin-${architecture}`;
            break;
        case 'linux':
            platformFolder = `linux-${architecture}`;
            break;
        case 'win32':
            platformFolder = 'win32-x64';
            break;
        default:
            throw new Error(`Unsupported platform: ${plat}`);
    }

    const exeName = process.platform === 'win32' ? 'coax.exe' : 'coax';
    return path.join(context.extensionPath, 'bundled', platformFolder, exeName);
}

/**
 * Check if a file should be skipped
 */
function shouldSkipFile(document: vscode.TextDocument): boolean {
    const config = vscode.workspace.getConfiguration('coax');
    const excludePatterns = config.get<string[]>('exclude', []);
    const maxFileSize = config.get<number>('maxFileSize', 10485760);

    // Check file size
    try {
        const stat = fs.statSync(document.uri.fsPath);
        if (stat.size > maxFileSize) {
            console.log(`Skipping ${document.fileName}: file too large (${stat.size} bytes)`);
            return true;
        }
    } catch (error) {
        // File might not exist on disk yet
    }

    // Check excluded patterns
    const filePath = document.uri.fsPath;
    for (const pattern of excludePatterns) {
        if (minimatch(filePath, pattern, { dot: true })) {
            console.log(`Skipping ${filePath}: matches exclude pattern ${pattern}`);
            return true;
        }
    }

    // Skip binary files
    const binaryExtensions = ['.bin', '.exe', '.dll', '.so', '.dylib', '.pdf', '.zip', '.tar', '.gz', '.png', '.jpg', '.jpeg', '.gif', '.bmp', '.svg', '.webp', '.ttf', '.woff', '.woff2'];
    const ext = path.extname(filePath).toLowerCase();
    if (binaryExtensions.includes(ext)) {
        return true;
    }

    // Skip minified files
    if (filePath.includes('.min.') || filePath.includes('bundle.')) {
        return true;
    }

    return false;
}

/**
 * Execute the Coax CLI binary
 */
async function executeBinary(args: string[]): Promise<string> {
    const binaryPath = getBinaryPath();
    const config = vscode.workspace.getConfiguration('coax');
    const timeout = config.get<number>('scanTimeout', 30000);

    // Set executable permissions on Unix
    if (process.platform !== 'win32') {
        try {
            fs.chmodSync(binaryPath, '755');
        } catch (error) {
            console.warn(`Failed to set executable permissions: ${error}`);
        }
    }

    return new Promise((resolve, reject) => {
        execFile(
            binaryPath,
            args,
            { timeout, encoding: 'utf8' },
            (error, stdout, stderr) => {
                if (error && error.code !== 'ENOENT') {
                    // Exit code 1 is OK (findings found)
                    if (error.signal !== 'SIGTERM' && !stderr.includes('no secrets found')) {
                        reject(new Error(`Binary execution failed: ${error.message}`));
                        return;
                    }
                }
                resolve(stdout);
            }
        );
    });
}

/**
 * Scan a file for secrets and vulnerabilities
 */
export async function scanFile(uri: vscode.Uri): Promise<Finding[]> {
    const document = await vscode.workspace.openTextDocument(uri);

    if (shouldSkipFile(document)) {
        return [];
    }

    const config = vscode.workspace.getConfiguration('coax');
    const unicodeEnabled = config.get<boolean>('unicode.enabled', true);
    const unicodeSensitivity = config.get<string>('unicode.sensitivity', 'high');

    try {
        const args = ['scan', '-p', document.uri.fsPath, '--format', 'json'];
        
        if (unicodeEnabled) {
            args.push('--unicode-only');
            args.push('--unicode-sensitivity', unicodeSensitivity);
        }

        const output = await executeBinary(args);
        
        if (!output.trim()) {
            return [];
        }

        const result: ScanResult = JSON.parse(output);
        return result.findings || [];
    } catch (error) {
        console.error(`Scan failed for ${document.fileName}:`, error);
        return [];
    }
}

/**
 * Scan the entire workspace
 */
export async function scanWorkspace(): Promise<Finding[]> {
    const config = vscode.workspace.getConfiguration('coax');
    const unicodeEnabled = config.get<boolean>('unicode.enabled', true);
    const unicodeSensitivity = config.get<string>('unicode.sensitivity', 'high');

    try {
        const args = ['scan', '-p', vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || '.', '--format', 'json'];
        
        if (unicodeEnabled) {
            args.push('--unicode-only');
            args.push('--unicode-sensitivity', unicodeSensitivity);
        }

        const output = await executeBinary(args);
        
        if (!output.trim()) {
            return [];
        }

        const result: ScanResult = JSON.parse(output);
        return result.findings || [];
    } catch (error) {
        console.error('Workspace scan failed:', error);
        return [];
    }
}
