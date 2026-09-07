// Worker that lists an R2 bucket and returns slideshow data as JSON.
// Bind the bucket as MEDIA_BUCKET and set PUBLIC_BASE_URL in wrangler.toml.

const IMAGE_EXTS = ['jpg', 'jpeg', 'png', 'gif'];
const VIDEO_EXTS = ['mp4'];

export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    // CORS preflight
    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders() });
    }

    if (url.pathname !== '/list') {
      return new Response('Not found', { status: 404 });
    }

    try {
      const objects = await listAll(env.MEDIA_BUCKET, 'photos/');
      const payload = buildPayload(objects, env.PUBLIC_BASE_URL);

      return new Response(JSON.stringify(payload), {
        headers: {
          'Content-Type': 'application/json',
          'Cache-Control': 'public, max-age=30', // short cache so updates show up quickly
          ...corsHeaders(),
        },
      });
    } catch (err) {
      return new Response(JSON.stringify({ error: String(err) }), {
        status: 500,
        headers: { 'Content-Type': 'application/json', ...corsHeaders() },
      });
    }
  },
};

function corsHeaders() {
  return {
    // Tighten this to your GitHub Pages origin once it's live, e.g.
    // 'Access-Control-Allow-Origin': 'https://yourusername.github.io',
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, OPTIONS',
  };
}

async function listAll(bucket, prefix) {
  let cursor;
  const objects = [];
  do {
    const listed = await bucket.list({ prefix, cursor, limit: 1000 });
    objects.push(...listed.objects);
    cursor = listed.truncated ? listed.cursor : undefined;
  } while (cursor);
  return objects;
}

function buildPayload(objects, publicBaseUrl) {
  const categories = new Map();
  const sponsors = [];

  for (const obj of objects) {
    const parts = obj.key.split('/'); // e.g. ["photos", "beach", "sunset.jpg"]
    if (parts[0] !== 'photos' || parts.length < 3) continue;

    const categoryName = parts[1];
    const fileName = parts[parts.length - 1];
    const ext = fileName.split('.').pop().toLowerCase();
    const fileUrl = `${publicBaseUrl}/${obj.key}`;

    if (categoryName === 'sponsors') {
      if (IMAGE_EXTS.includes(ext)) sponsors.push(fileUrl);
      continue;
    }

    if (!categories.has(categoryName)) {
      categories.set(categoryName, { name: categoryName, images: [], videos: [] });
    }
    const cat = categories.get(categoryName);

    if (VIDEO_EXTS.includes(ext)) cat.videos.push(fileUrl);
    else if (IMAGE_EXTS.includes(ext)) cat.images.push(fileUrl);
  }

  return { categories: Array.from(categories.values()), sponsors };
}
