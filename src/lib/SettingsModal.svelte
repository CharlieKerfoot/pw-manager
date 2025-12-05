<script>
	import Icon from '@iconify/svelte';

	let { showSettings = $bindable(), onClearData } = $props();

	function handleExport() {
		fetch(`${import.meta.env.VITE_API_URL}/accounts`, {
			headers: { 'Authorization': `Bearer ${localStorage.getItem('token')}` }
		})
			.then(res => res.json())
			.then(data => {
				const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
				const url = URL.createObjectURL(blob);
				const a = document.createElement('a');
				a.href = url;
				a.download = 'tpm_vault_export.json';
				a.click();
				URL.revokeObjectURL(url);
			});
	}

	async function handleDeleteAll() {
		if (confirm('WARNING: THIS WILL PERMANENTLY ERASE ALL DATA. CONFIRM?')) {
			try {
				await fetch(`${import.meta.env.VITE_API_URL}/accounts/all`, {
					method: 'DELETE',
					headers: { 'Authorization': `Bearer ${localStorage.getItem('token')}` }
				});
				onClearData();
				showSettings = false;
			} catch (e) {
				console.error('Failed to delete data', e);
			}
		}
	}
</script>

<dialog open={showSettings}>
	<div class="modal-content">
		<div class="modal-header">
			<h2>SYSTEM_CONFIG</h2>
			<button class="close-btn" onclick={() => (showSettings = false)}>[X]</button>
		</div>

		<div class="settings-section">
			<h3>ABOUT_SYSTEM</h3>
			<div class="info-row">
				<span>VERSION</span>
				<span class="value">2.0.0-KINETIC</span>
			</div>
			<div class="info-row">
				<span>BUILD</span>
				<span class="value">STABLE</span>
			</div>
		</div>

		<div class="settings-section">
			<h3>DATA_MANAGEMENT</h3>
			<div class="actions-grid">
				<button class="action-btn export" onclick={handleExport}>
					<Icon icon="material-symbols:download" />
					EXPORT_DATABASE
				</button>
				<button class="action-btn delete" onclick={handleDeleteAll}>
					<Icon icon="material-symbols:delete-forever" />
					PURGE_ALL_DATA
				</button>
			</div>
		</div>
	</div>
</dialog>

<style>
	dialog {
		background: rgba(0, 0, 0, 0.8);
		border: none;
		width: 100%;
		height: 100%;
		position: fixed;
		top: 0;
		left: 0;
		display: none;
		justify-content: center;
		align-items: center;
		z-index: 1000;
		padding: 0;
		margin: 0;
	}

	dialog[open] {
		display: flex;
	}

	.modal-content {
		background: var(--color-bg);
		border: 2px solid var(--color-accent);
		padding: 2em;
		width: 100%;
		max-width: 500px;
		box-shadow: var(--shadow-hard);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2em;
		border-bottom: 1px solid var(--color-border);
		padding-bottom: 1em;
	}

	h2 {
		color: var(--color-accent);
		font-size: 1.5em;
		margin: 0;
	}

	.close-btn {
		background: transparent;
		border: none;
		color: #666;
		font-size: 1.2em;
		padding: 0;
	}

	.close-btn:hover {
		color: var(--color-danger);
	}

	.settings-section {
		margin-bottom: 2em;
	}

	h3 {
		font-size: 0.9em;
		color: #666;
		margin-bottom: 1em;
		letter-spacing: 1px;
	}

	.info-row {
		display: flex;
		justify-content: space-between;
		padding: 0.8em 0;
		border-bottom: 1px solid var(--color-border);
		font-size: 0.9em;
	}

	.value {
		color: var(--color-accent);
		font-family: var(--font-mono);
	}

	.actions-grid {
		display: grid;
		gap: 1em;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5em;
		padding: 1em;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		color: white;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: white;
		color: black;
	}
</style>
