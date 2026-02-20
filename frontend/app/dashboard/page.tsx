export default function DashboardPage() {
  return (
    <section className="rounded-3xl border border-white/10 bg-white/[0.04] p-6 backdrop-blur-xl md:p-8">
      <p className="font-body text-xs tracking-[0.12em] text-white/60 uppercase">
        Dashboard
      </p>
      <h1 className="font-heading mt-2 text-3xl md:text-5xl">Your Streams</h1>
      <p className="font-body mt-4 max-w-2xl text-white/72">
        Monitor active flows, track settlements in real time, and manage your
        streaming positions from one command center.
      </p>
    </section>
  );
}
