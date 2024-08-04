// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

class ExtensionHostProcessEnvProvider implements vscode.TreeDataProvider<EnvVar> {
	constructor(protected readonly workspaceFolders?: readonly vscode.WorkspaceFolder[]) {}

	getTreeItem(element: EnvVar): vscode.TreeItem {
		return element;
	}

	getChildren(element?: EnvVar): Thenable<EnvVar[]> {
		if (!element) {
			const children: EnvVar[] = [];
			const sorted = Object.entries(process.env).sort((a, b) => a[0] > b[0] ? 1 : b[0] > a[0] ? -1 : 0);

			for (const [name, value] of sorted) {
				if (name === "PATH") {
					children.push(new PathEnvVar(name, value?.split(":") ?? []));
				} else {
					children.push(new SimpleEnvVar(name, value));
				}
			}

			return Promise.resolve(children);
		} else if (isPathEnvVar(element)) {
			return Promise.resolve((element.value).map((value) => new SimpleEnvVar("", value)));
		} else {
			return Promise.resolve([]);
		}
	}
}

type EnvVar = SimpleEnvVar | PathEnvVar;
class SimpleEnvVar extends vscode.TreeItem {
	constructor(public readonly name: string, public readonly value: string | null | undefined) {
		super(name, vscode.TreeItemCollapsibleState.None);
		this.tooltip = `${name}=${value}`;
		this.description = value === null ? 'null' : value === undefined ? 'undefined' : value;
	}
}

class PathEnvVar extends vscode.TreeItem {
	constructor(public readonly name: string, public readonly value: string[], collapsedState?: vscode.TreeItemCollapsibleState.Collapsed) {
		super(name, collapsedState ?? vscode.TreeItemCollapsibleState.Collapsed);
		this.tooltip = `${name}[${value.length}]`;
		this.description = value.join(":");
	}
}

function isPathEnvVar(a: EnvVar): a is PathEnvVar {
	return a instanceof PathEnvVar;
}


// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "vscode-env" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	const commandDisposable = vscode.commands.registerCommand('vscode-env.helloWorld', () => {
		// The code you place here will be executed every time your command is executed
		// Display a message box to the user
		vscode.window.showInformationMessage('Hello World from env!');
	});

	context.subscriptions.push(commandDisposable);

	vscode.window.createTreeView('env', {
		treeDataProvider: new ExtensionHostProcessEnvProvider(vscode.workspace.workspaceFolders),
	});
}

// This method is called when your extension is deactivated
export function deactivate() {}
