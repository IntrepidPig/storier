var renderPreview = function () {
	$('#title-preview').html($('#post-title').val());
	$('#body-preview').html($('#post-content').val());
	$('#date-preview').html(moment().format("ddd/D MMMM YYYY<br>h:mm A"));
}

function sleep(s) {
	return new Promise(resolve => setTimeout(resolve, s * 1000));
}

async function keepTime() {
	while(true) {
		await sleep(15);
		renderPreview();
	}
}

$(document).ready(function () {
	renderPreview();
	$('#post-title').bind('input', renderPreview);
	$('#post-content').bind('input', renderPreview);

	keepTime();
});